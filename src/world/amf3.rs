//! (De-)serialization support for the [AMF3 format](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/pdf/amf-file-format-spec.pdf).
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::io::{Error, ErrorKind::InvalidData, Read, Result as Res, Write};
use std::ops::{Index, IndexMut};

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};
use lu_packets_derive::GmParam;

struct Amf3Reader<'a, R: Read> {
	inner: &'a mut R,
	string_ref_table: Vec<Amf3String>,
}

impl<R: Read> Read for Amf3Reader<'_, R> {
	fn read(&mut self, buf: &mut [u8]) -> Res<usize> {
		self.inner.read(buf)
	}
}

struct Amf3Writer<'a, W: Write> {
	inner: &'a mut W,
	string_ref_table: Vec<Amf3String>,
}

impl<W: Write> Write for Amf3Writer<'_, W> {
	fn write(&mut self, buf: &[u8]) -> Res<usize> {
		self.inner.write(buf)
	}

	fn flush(&mut self) -> Res<()> {
		self.inner.flush()
	}
}

/// An error returned when an integer is too large to be represented by an `U29`.
#[derive(Debug)]
pub struct U29Error;

/// An unsigned integer whose serialized form has variable length, with a maximum value of 2^29-1. [`See spec section 1.3.1`](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/pdf/amf-file-format-spec.pdf#%5B%7B%22num%22%3A18%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C88%2C643%2C0%5D).
#[derive(Debug)]
struct U29(u32);

/// Converts the value to a [`U29`] if it is less than 2^29, returns [`U29Error`] otherwise.
impl TryFrom<usize> for U29 {
	type Error = U29Error;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		if value >= 1 << 29 {
			Err(U29Error)
		} else {
			Ok(Self(value as u32))
		}
	}
}

impl<R: Read> Deserialize<LE, R> for U29 {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let mut value = 0;
		for _ in 0..3 {
			let byte: u8 = LERead::read(reader)?;
			let byte = byte as u32;
			value = (value << 7) | (byte & 0x7f);
			if byte & 0x80 == 0 {
				return Ok(Self(value));
			}
		}
		let byte: u8 = LERead::read(reader)?;
		let byte = byte as u32;
		value = (value << 8) | byte;
		Ok(Self(value))
	}
}

impl<'a, W: Write> Serialize<LE, W> for &'a U29 {
	fn serialize(self, writer: &mut W) -> Res<()> {
		let v = self.0;
		if v <= 0x7f {
			LEWrite::write(writer, v as u8)
		} else if v <= 0x3fff {
			LEWrite::write(writer, (v >> 7) as u8 | 0x80)?;
			LEWrite::write(writer, v as u8 & 0x7f)
		} else if v <= 0x1fffff {
			LEWrite::write(writer, (v >> 14) as u8 | 0x80)?;
			LEWrite::write(writer, (v >> 7 ) as u8 | 0x80)?;
			LEWrite::write(writer, v as u8 & 0x7f)
		} else {
			LEWrite::write(writer, (v >> 22) as u8 | 0x80)?;
			LEWrite::write(writer, (v >> 15) as u8 | 0x80)?;
			LEWrite::write(writer, (v >> 8 ) as u8 | 0x80)?;
			LEWrite::write(writer, v as u8)
		}
	}
}

/**
	A string with a maximum length of 2^29-1.

	[See spec section 3.8 for more](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/pdf/amf-file-format-spec.pdf#%5B%7B%22num%22%3A22%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C88%2C196%2C0%5D).
*/
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Amf3String(String);

impl Debug for Amf3String {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		self.0.fmt(f)
	}
}

impl Borrow<str> for Amf3String {
	fn borrow(&self) -> &str {
		&self.0[..]
	}
}

impl TryFrom<&str> for Amf3String {
	type Error = U29Error;

	fn try_from(string: &str) -> Result<Self, Self::Error> {
		if string.len() & (1 << 29) != 0 {
			Err(U29Error)
		} else {
			Ok(Self(string.into()))
		}
	}
}

impl<R: Read> Deserialize<LE, Amf3Reader<'_, R>> for Amf3String {
	fn deserialize(reader: &mut Amf3Reader<'_, R>) -> Res<Self> {
		let value_and_is_inline: U29 = LERead::read(reader)?;
		let is_inline = value_and_is_inline.0 & 0x01 == 1;
		let value = value_and_is_inline.0 >> 1;
		let string = if !is_inline {
			let index = value;
			match reader.string_ref_table.get(index as usize) {
				Some(x) => x.0.clone(),
				None => { return Err(Error::new(InvalidData, "invalid reference index")) }
			}
		} else {
			let length = value;

			let mut vec = vec![0u8; length as usize];
			Read::read_exact(reader, &mut vec)?;

			let string = match String::from_utf8(vec) {
				Ok(x) => x,
				Err(_) => { return Err(Error::new(InvalidData, "string is not valid utf8")) }
			};
			if string != "" {
				reader.string_ref_table.push(Self(string.clone()));
			}
			string
		};

		Ok(Self(string))
	}
}

impl<'a, W: Write> Serialize<LE, Amf3Writer<'_, W>> for &'a Amf3String {
	fn serialize(self, writer: &mut Amf3Writer<'_, W>) -> Res<()> {
		if self.0 == "" {
			let length_and_is_inline = U29(1);
			return LEWrite::write(writer, &length_and_is_inline);
		}
		match writer.string_ref_table.iter().position(|x| x == self) {
			Some(index) => {
				let index_and_is_inline = U29((index as u32) << 1);
				LEWrite::write(writer, &index_and_is_inline)
			}
			None => {
				let length_and_is_inline = U29((self.0.len() as u32) << 1 | 1);
				LEWrite::write(writer, &length_and_is_inline)?;
				writer.string_ref_table.push(Amf3String(self.0.clone()));
				Write::write_all(writer, self.0.as_bytes())
			}
		}
	}
}

/**
	Both a dense and associative array at the same time.

	[See spec section 3.11 for more](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/pdf/amf-file-format-spec.pdf#%5B%7B%22num%22%3A24%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C88%2C720%2C0%5D).
*/
#[derive(Clone, PartialEq)]
pub struct Amf3Array {
	pub map: HashMap<Amf3String, Amf3>,
	pub vec: Vec<Amf3>,
}

impl Amf3Array {
	pub fn new() -> Self {
		Self { map: HashMap::new(), vec: vec![] }
	}
}

impl Debug for Amf3Array {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let map_empty = self.map.is_empty();
		let vec_empty = self.vec.is_empty();
		if !map_empty && vec_empty {
			write!(f, "amf3! ")?;
			self.map.fmt(f)
		} else if map_empty && !vec_empty {
			write!(f, "amf3! ")?;
			self.vec.fmt(f)
		} else if map_empty && vec_empty {
			write!(f, "amf3! {{}}")
		} else {
			write!(f, "Amf3Array {{ map: {:?}, vec: {:?} }}", self.map, self.vec)
		}
	}
}

impl Index<usize> for Amf3Array {
	type Output = Amf3;

	fn index(&self, index: usize) -> &Self::Output {
		&self.vec[index]
	}
}

impl IndexMut<usize> for Amf3Array {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.vec[index]
	}
}

impl Index<&Amf3String> for Amf3Array {
	type Output = Amf3;

	fn index(&self, key: &Amf3String) -> &Amf3 {
		&self.map[key]
	}
}

impl Index<&str> for Amf3Array {
	type Output = Amf3;

	fn index(&self, key: &str) -> &Amf3 {
		&self.map[key]
	}
}

impl<R: Read> Deserialize<LE, Amf3Reader<'_, R>> for Amf3Array {
	fn deserialize(reader: &mut Amf3Reader<'_, R>) -> Res<Self> {
		let length_and_is_inline: U29 = LERead::read(reader)?;
		let is_inline = length_and_is_inline.0 & 0x01 == 1;
		let length = length_and_is_inline.0 >> 1;
		if !is_inline { todo!() }
		let mut map = HashMap::new();
		loop {
			let key: Amf3String = LERead::read(reader)?;
			if key.0 == "" {
				break;
			}
			let value = deser_amf3(reader)?;
			map.insert(key, value);
		}
		let mut vec = Vec::with_capacity(length as usize);
		for _ in 0..length {
			let value = deser_amf3(reader)?;
			vec.push(value);
		}

		Ok(Self { map, vec })
	}
}

impl<'a, W: Write> Serialize<LE, Amf3Writer<'_, W>> for &'a Amf3Array {
	fn serialize(self, writer: &mut Amf3Writer<'_, W>) -> Res<()> {
		let length_and_is_inline = U29((self.vec.len() as u32) << 1 | 1);
		LEWrite::write(writer, &length_and_is_inline)?;
		#[cfg(test)]
		let key_value = {
			let mut key_value: Vec<_> = self.map.iter().collect();
			key_value.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
			key_value
		};
		#[cfg(not(test))]
		let key_value = self.map.iter();
		for (key, value) in key_value {
			LEWrite::write(writer, key)?;
			ser_amf3(writer, value)?;
		}
		LEWrite::write(writer, &Amf3String("".into()))?;
		for value in &self.vec {
			ser_amf3(writer, value)?;
		}
		Ok(())
	}
}
/**
	A type that can be (de-)serialized in the AMF3 format.

	[See spec section 3 for more](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/pdf/amf-file-format-spec.pdf#%5B%7B%22num%22%3A20%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C88%2C305%2C0%5D).
*/
#[derive(Clone, GmParam, PartialEq)]
#[repr(u8)]
pub enum Amf3 {
	False = 2,
	True = 3,
	Double(f64) = 5,
	String(Amf3String) = 6,
	Array(Amf3Array) = 9,
}

impl Debug for Amf3 {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::False     => write!(f, "false"),
			Self::True      => write!(f, "true"),
			Self::Double(x) => x.fmt(f),
			Self::String(x) => x.fmt(f),
			Self::Array (x) => x.fmt(f),
		}
	}
}

impl<R: Read> Deserialize<LE, R> for Amf3 {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let mut reader = Amf3Reader { inner: reader, string_ref_table: vec![] };
		deser_amf3(&mut reader)
	}
}

fn deser_amf3<R: Read>(reader: &mut Amf3Reader<R>) -> Res<Amf3> {
	let disc: u8 = LERead::read(reader)?;
	Ok(match disc {
		2 => Amf3::False,
		3 => Amf3::True,
		5 => Amf3::Double(LERead::read(reader)?),
		6 => Amf3::String(LERead::read(reader)?),
		9 => Amf3::Array (LERead::read(reader)?),
		_ => { return Err(Error::new(InvalidData, format!("invalid discriminant value for Amf3: {}", disc))) }
	})
}

impl<'a, W: Write> Serialize<LE, W> for &'a Amf3 {
	fn serialize(self, writer: &mut W) -> Res<()> {
		let mut writer = Amf3Writer { inner: writer, string_ref_table: vec![] };
		ser_amf3(&mut writer, self)
	}
}

fn ser_amf3<W: Write>(writer: &mut Amf3Writer<W>, amf3: &Amf3) -> Res<()> {
	match amf3 {
		Amf3::False     =>   LEWrite::write(writer, 2u8),
		Amf3::True      =>   LEWrite::write(writer, 3u8),
		Amf3::Double(x) => { LEWrite::write(writer, 5u8)?; LEWrite::write(writer, x) },
		Amf3::String(x) => { LEWrite::write(writer, 6u8)?; LEWrite::write(writer, x) },
		Amf3::Array (x) => { LEWrite::write(writer, 9u8)?; LEWrite::write(writer, x) },
	}
}

impl From<bool> for Amf3 {
	fn from(b: bool) -> Self {
		if b { Self::True } else { Self::False }
	}
}

impl From<f32> for Amf3 {
	fn from(f: f32) -> Self {
		Self::Double(f.into())
	}
}

impl From<f64> for Amf3 {
	fn from(f: f64) -> Self {
		Self::Double(f)
	}
}

impl TryFrom<&str> for Amf3 {
	type Error = U29Error;

	fn try_from(string: &str) -> Result<Self, Self::Error> {
		Ok(Self::String(string.try_into()?))
	}
}

#[cfg(test)]
mod tests {
	use endio::{LERead, LEWrite};
	use super::U29;

	#[test]
	fn test_u29() {
		for (bytes, integer) in &[(&b"\x7f"[..], 0x7f), (&b"\xa2\x43"[..], 4419), (&b"\x88\x00"[..], 1024), (&b"\xff\xff\x7e"[..], 0x1ffffe), (&b"\x80\xc0\x80\x00"[..], 0x200000), (&b"\xbf\xff\xff\xfe"[..], 0xffffffe)] {
			let mut reader = &bytes[..];
			let val: U29 = reader.read().unwrap();
			assert_eq!(val.0, *integer);
			let mut writer = vec![];
			writer.write(&val).unwrap();
			assert_eq!(&&writer[..], bytes);
		}
	}
}
