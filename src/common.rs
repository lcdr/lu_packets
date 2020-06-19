use std::convert::{TryFrom, TryInto};
use std::io::{Error, ErrorKind::InvalidData, Read, Write};
use std::io::Result as Res;
use std::marker::PhantomData;
use std::net::Ipv4Addr;

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};

pub(crate) fn err<T, U: std::fmt::Debug>(name: &str, value: U) -> Res<T> {
	Err(Error::new(InvalidData, &format!("unknown {} {:?}", name, value)[..]))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemAddress {
	ip: Ipv4Addr,
	port: u16,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[repr(u16)]
pub enum ServiceId {
	General = 0,
	Auth = 1,
	Chat = 2,
	World = 4,
	Client = 5,
}

macro_rules! lu_str {
	($name:ident, $n:literal) => {
		// todo: runtime type invariants checks (valid ascii, null terminator)
		/// A string encoded in ASCII with a maximum length of $n.
		pub struct $name([u8; $n]);

		impl $name {
			fn get_str(&self) -> &str {
				let terminator = self.0.iter().position(|&c| c == 0).unwrap();
				std::str::from_utf8(&self.0[..terminator]).unwrap()
			}
		}

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
				let s: &str = self.get_str();
				write!(f, "{:?}", s)
			}
		}

		impl From<&str> for $name {
			fn from(string: &str) -> Self {
				let mut bytes = [0u8; $n];
				for (i, chr) in string.bytes().take($n-1).enumerate() {
					bytes[i] = chr;
				}
				Self(bytes)
			}
		}

		impl<R: Read> Deserialize<LE, R> for $name {
			fn deserialize(reader: &mut R) -> Res<Self> {
				let mut bytes = [0u8; $n];
				reader.read(&mut bytes)?;
				Ok(Self(unsafe { std::mem::transmute(bytes) }))
			}
		}
		impl<W: Write> Serialize<LE, W> for &$name {
			fn serialize(self, writer: &mut W) -> Res<()> {
				writer.write_all(&self.0)
			}
		}
	}
}

macro_rules! lu_wstr {
	($name:ident, $n:literal) => {
		// todo: runtime type invariants checks (valid ucs-2, null terminator)
		/// A string encoded in UCS-2 with a maximum length of $n.
		pub struct $name([u16; $n]);

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
				write!(f, "{:?}", String::from(self))
			}
		}

		impl From<&str> for $name {
			fn from(string: &str) -> Self {
				let mut bytes = [0u16; $n];
				for (i, chr) in string.encode_utf16().take($n-1).enumerate() {
					bytes[i] = chr;
				}
				Self(bytes)
			}
		}

		impl From<&$name> for String {
			fn from(wstr: &$name) -> Self {
				let terminator = wstr.0.iter().position(|&c| c == 0).unwrap();
				String::from_utf16(&wstr.0[..terminator]).unwrap()
			}
		}

		impl<R: Read> Deserialize<LE, R> for $name {
			fn deserialize(reader: &mut R) -> Res<Self> {
				let mut bytes = [0u8; $n*2];
				reader.read(&mut bytes)?;
				Ok(Self(unsafe { std::mem::transmute(bytes) }))
			}
		}

		impl<W: Write> Serialize<LE, W> for &$name {
			fn serialize(self, writer: &mut W) -> Res<()> {
				let x: [u8; $n*2] = unsafe { std::mem::transmute(self.0) };
				writer.write_all(&x)
			}
		}
	}
}

lu_str!(LuStr33, 33);
lu_wstr!(LuWStr33, 33);
lu_wstr!(LuWStr41, 41);
lu_wstr!(LuWStr42, 42);
lu_wstr!(LuWStr128, 128);
lu_wstr!(LuWStr256, 256);

pub struct LuVarStr<L>(Vec<u8>, PhantomData<L>);

impl<L> std::fmt::Debug for LuVarStr<L> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		let str_ = unsafe { std::str::from_utf8_unchecked(&self.0) };
		write!(f, "{:?}", str_)
	}
}

impl<L, R: Read> Deserialize<LE, R> for LuVarStr<L>
	where L: TryInto<usize> + Deserialize<LE, R> {

	fn deserialize(reader: &mut R) -> Res<Self> {
		let str_len: L = LERead::read(reader)?;
		let str_len = match str_len.try_into() {
			Ok(x) => x,
			_ => panic!(),
		};
		let mut vec = Vec::with_capacity(str_len);
		unsafe { vec.set_len(str_len); }
		Read::read_exact(reader, &mut vec)?;
		Ok(Self(vec, PhantomData))
	}
}

impl<'a, L, W: Write> Serialize<LE, W> for &'a LuVarStr<L>
	where L: TryFrom<usize> + Serialize<LE, W> {

	fn serialize(self, writer: &mut W) -> Res<()> {
		let l_len = match L::try_from(self.0.len()) {
			Ok(x) => x,
			_ => panic!(),
		};
		LEWrite::write(writer, l_len)?;
		Write::write_all(writer, &self.0)
	}
}

pub struct LuVarWStr<L>(Vec<u16>, PhantomData<L>);

impl<L> std::fmt::Debug for LuVarWStr<L> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "{:?}", String::from(self))
	}
}

impl<L> From<&LuVarWStr<L>> for String {
	fn from(wstr: &LuVarWStr<L>) -> String {
		String::from_utf16(&wstr.0).unwrap()
	}
}

impl<L, R: Read> Deserialize<LE, R> for LuVarWStr<L>
	where L: TryInto<usize> + Deserialize<LE, R> {

	fn deserialize(reader: &mut R) -> Res<Self> {
		let str_len: L = LERead::read(reader)?;
		let str_len = match str_len.try_into() {
			Ok(x) => x,
			_ => panic!(),
		};
		let mut ucs2_str = Vec::<u16>::with_capacity(str_len);
		unsafe {
			ucs2_str.set_len(str_len);
			let mut ucs2_str_slice = std::slice::from_raw_parts_mut(ucs2_str.as_mut_ptr() as *mut u8, str_len*2);
			Read::read(reader, &mut ucs2_str_slice)?;
		}
		Ok(Self(ucs2_str, PhantomData))
	}
}

impl<'a, L, W: Write> Serialize<LE, W> for &'a LuVarWStr<L>
	where L: TryFrom<usize> + Serialize<LE, W> {

	fn serialize(self, writer: &mut W) -> Res<()> {
		let str_len = self.0.len();
		let l_len = match L::try_from(str_len) {
			Ok(x) => x,
			_ => panic!(),
		};
		LEWrite::write(writer, l_len)?;
		let u8_slice = unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u8, str_len*2) };
		Write::write_all(writer, u8_slice)
	}
}

pub type ObjId = u64;
pub const OBJID_EMPTY: u64 = 0;
