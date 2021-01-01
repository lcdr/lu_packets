//! LU name value datatype.
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Write};
use std::io::Result as Res;

use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};
use super::gm::GmParam;

use crate::common::{LuStrExt, LuVarString, LuVarWString, LuWStr};

/// A value contained in a [`LuNameValue`].
#[derive(Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum LnvValue {
	WString(LuVarWString<u32>) = 0,
	I32(i32) = 1,
	F32(f32) = 3,
	F64(f64) = 4,
	U32(u32) = 5,
	Bool(bool) = 7,
	I64(i64) = 8,
	U64(u64) = 9,
	String(LuVarString<u32>) = 13,
}

impl LnvValue {
	fn parse_ty_val(wstr: &LuWStr) -> Self {
		let string: String = wstr.to_string();
		let (ty, val) = string.split_at(string.find(":").unwrap());
		let val = val.split_at(1).1;
		match ty {
			"0"  => LnvValue::WString(val.try_into().unwrap()),
			"1"  => LnvValue::I32(val.parse().unwrap()),
			"3"  => LnvValue::F32(val.parse().unwrap()),
			"4"  => LnvValue::F64(val.parse().unwrap()),
			"5"  => LnvValue::U32(val.parse().unwrap()),
			"7"  => LnvValue::Bool(val == "1"),
			"8"  => LnvValue::I64(val.parse().unwrap()),
			"9"  => LnvValue::U64(val.parse().unwrap()),
			"13" => LnvValue::String(val.as_bytes().try_into().unwrap()),
			_ => panic!(),
		}
	}
}

impl std::fmt::Debug for LnvValue {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		match self {
			LnvValue::WString(x) => write!(f, "{:?}", x),
			LnvValue::I32    (x) => write!(f, "{:?}i32", x),
			LnvValue::F32    (x) => write!(f, "{:?}f32", x),
			LnvValue::F64    (x) => write!(f, "{:?}f64", x),
			LnvValue::U32    (x) => write!(f, "{:?}u32", x),
			LnvValue::Bool   (x) => write!(f, "{:?}", x),
			LnvValue::I64    (x) => write!(f, "{:?}i64", x),
			LnvValue::U64    (x) => write!(f, "{:?}u64", x),
			LnvValue::String (x) => write!(f, "{:?}", x),
		}
	}
}

impl From<LuVarWString<u32>> for LnvValue {
	fn from(val: LuVarWString<u32>) -> Self { LnvValue::WString(val) }
}

impl From<&str> for LnvValue {
	fn from(val: &str) -> Self { LnvValue::WString(val.try_into().unwrap()) }
}

impl From<i32> for LnvValue {
	fn from(val: i32) -> Self { LnvValue::I32(val) }
}

impl From<f32> for LnvValue {
	fn from(val: f32) -> Self { LnvValue::F32(val) }
}

impl From<f64> for LnvValue {
	fn from(val: f64) -> Self { LnvValue::F64(val) }
}

impl From<u32> for LnvValue {
	fn from(val: u32) -> Self { LnvValue::U32(val) }
}

impl From<bool> for LnvValue {
	fn from(val: bool) -> Self { LnvValue::Bool(val) }
}

impl From<i64> for LnvValue {
	fn from(val: i64) -> Self { LnvValue::I64(val) }
}

impl From<u64> for LnvValue {
	fn from(val: u64) -> Self { LnvValue::U64(val) }
}

impl From<&[u8]> for LnvValue {
	fn from(val: &[u8]) -> Self { LnvValue::String(val.try_into().unwrap()) }
}

impl<const N: usize> From<&[u8; N]> for LnvValue {
	fn from(val: &[u8; N]) -> Self { LnvValue::String(val.try_into().unwrap()) }
}

/// A hash map with values being one of multiple possible types.
#[derive(PartialEq)]
pub struct LuNameValue(HashMap<LuVarWString<u32>, LnvValue>);

impl LuNameValue {
	pub fn new() -> Self {
		LuNameValue(HashMap::new())
	}
}

impl std::ops::Deref for LuNameValue {
	type Target = HashMap<LuVarWString<u32>, LnvValue>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for LuNameValue {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl std::fmt::Debug for LuNameValue {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "lnv! ")?;
		f.debug_map().entries(self.0.iter().map(|(k, v)| (k, v))).finish()
	}
}

impl<R: Read> Deserialize<LE, R> for LuNameValue {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let len: u32 = LERead::read(reader)?;
		let is_compressed: bool = LERead::read(reader)?;
		let uncompressed = if is_compressed {
			let uncomp_len: u32 = LERead::read(reader)?;
			let comp_len: u32 = LERead::read(reader)?;
			let mut comp = vec![0; comp_len as usize];
			reader.read_exact(&mut comp)?;
			let mut inflater = ZlibDecoder::new(&comp[..]);
			let mut uncomp = Vec::with_capacity(uncomp_len as usize);
			inflater.read_to_end(&mut uncomp)?;
			assert_eq!(uncomp.len(), uncomp_len as usize);
			uncomp
		} else {
			let mut uncomp = vec![0; len as usize -1];
			reader.read_exact(&mut uncomp)?;
			uncomp
		};
		let unc_reader = &mut &uncompressed[..];
		let lnv_len: u32 = LERead::read(unc_reader)?;
		let mut res = lnv!();
		for _ in 0..lnv_len {
			let enc_key_len: u8 = LERead::read(unc_reader)?;
			let key = LuVarWString::deser_content(unc_reader, enc_key_len as u32 / 2)?;
			let value = LERead::read(unc_reader)?;
			res.insert(key, value);
		}
		Ok(res)
	}
}

impl<'a, W: Write> Serialize<LE, W> for &'a LuNameValue {
	fn serialize(self, writer: &mut W) -> Res<()> {
		let mut uncompressed: Vec<u8> = vec![];
		LEWrite::write(&mut uncompressed, self.len() as u32)?;
		#[cfg(test)]
		let key_value = {
			let mut key_value: Vec<_> = self.0.iter().collect();
			key_value.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
			key_value
		};
		#[cfg(not(test))]
		let key_value = self.0.iter();
		for (key, value) in key_value {
			LEWrite::write(&mut uncompressed, key.len() as u8 * 2)?;
			key.ser_content(&mut uncompressed)?;
			LEWrite::write(&mut uncompressed, value)?;
		}
		let is_compressed = true;
		if !is_compressed {
			LEWrite::write(writer, uncompressed.len() as u32 + 1)?;
			LEWrite::write(writer, false)?;
			writer.write_all(&uncompressed)?;
		} else {
			let mut compressed = vec![];
			ZlibEncoder::new(&mut compressed, Compression::new(6)).write_all(&uncompressed)?;
			LEWrite::write(writer, compressed.len() as u32 + 1 + 4 + 4)?;
			LEWrite::write(writer, true)?;
			LEWrite::write(writer, uncompressed.len() as u32)?;
			LEWrite::write(writer, compressed.len() as u32)?;
			writer.write_all(&compressed)?;
		}
		Ok(())
	}
}

impl From<&LuVarWString<u32>> for LuNameValue {
	fn from(wstr: &LuVarWString<u32>) -> Self {
		if wstr.is_empty() {
			return LuNameValue(HashMap::new())
		}
		let mut map = HashMap::new();
		for name_type_val in wstr.split(|c| *c == b'\n'.into()) {
			let (name, type_val) = name_type_val.split_at(name_type_val.iter().position(|c| *c == b'='.into()).unwrap());
			let name: LuVarWString<u32> = name.into();
			let type_val = type_val.split_at(1).1;
			let lnv_value = LnvValue::parse_ty_val(type_val);
			map.insert(name, lnv_value);
		}
		LuNameValue(map)
	}
}

impl From<&LuNameValue> for LuVarWString<u32> {
	fn from(lnv: &LuNameValue) -> Self {
		let mut wstr: Self = vec![].into();
		let mut i = 0;
		let len = lnv.0.len();
		let mut key_value: Vec<_> = lnv.0.iter().collect();
		key_value.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
		for (key, value) in key_value {
			wstr.extend_from_slice(&key);
			wstr.push(b'='.into());
			let (disc, val_str) = match value {
				LnvValue::WString(val) => ("0",  val.to_string()),
				LnvValue::I32    (val) => ("1",  val.to_string()),
				LnvValue::F32    (val) => ("3",  val.to_string()),
				LnvValue::F64    (val) => ("4",  val.to_string()),
				LnvValue::U32    (val) => ("5",  val.to_string()),
				LnvValue::Bool   (val) => ("7",  (*val as u8).to_string()),
				LnvValue::I64    (val) => ("8",  val.to_string()),
				LnvValue::U64    (val) => ("9",  val.to_string()),
				LnvValue::String (val) => ("13", val.to_string()),
			};
			wstr.extend_from_slice(&LuVarWString::<u32>::try_from(disc).unwrap());
			wstr.push(b':'.into());
			wstr.extend_from_slice(&LuVarWString::<u32>::try_from(val_str.as_str()).unwrap());
			i += 1;
			if i < len {
				wstr.push(b'\n'.into());
			}
		}
		wstr
	}
}

impl GmParam for LuNameValue {
	fn deserialize<R: ::std::io::Read>(reader: &mut R) -> ::std::io::Result<Self> {
		let lu_var_wstr: LuVarWString<u32> = LERead::read(reader)?;
		if !lu_var_wstr.is_empty() {
			let _: u16 = LERead::read(reader)?; // for some reason has a null terminator
		}
		Ok((&lu_var_wstr).into())
	}

	fn serialize<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
		let lu_var_wstr: LuVarWString<u32> = self.into();
		LEWrite::write(writer, &lu_var_wstr)?;
		if !lu_var_wstr.is_empty() {
			LEWrite::write(writer, 0u16)?; // for some reason has a null terminator
		}
		Ok(())
	}
}
