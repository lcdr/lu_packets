use std::io::{Error, ErrorKind::InvalidData};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use endio::LittleEndian as LE;

pub(crate) fn err<T>(msg: &str) -> Res<T> {
	Err(Error::new(InvalidData, msg))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ServiceId {
	General = 0,
	Auth = 1,
	Client = 5,
}

impl<R: LERead> Deserialize<LE, R> for ServiceId
	where u16: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let id: u16 = reader.read()?;
		Ok(match id {
			x if x == ServiceId::General as u16 => ServiceId::General,
			x if x == ServiceId::Auth as u16 => ServiceId::Auth,
			x if x == ServiceId::Client as u16 => ServiceId::Client,
			_ => {
				return err("unknown service id");
			}
		})
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for ServiceId
	where u16: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self as u16)
	}
}

macro_rules! lu_str {
	($name:ident, $n:literal) => {
		// todo: runtime type invariants checks (valid ascii, null terminator)
		pub struct $name(pub [u8; $n]);

		impl $name {
			fn get_str(&self) -> &str {
				let terminator = self.0.iter().position(|&c| c == 0).unwrap();
				std::str::from_utf8(&self.0[..terminator]).unwrap()
			}
		}

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
				let s: &str = self.get_str();
				write!(f, "{}", s)
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

		impl<R: std::io::Read> endio::Deserialize<LE, R> for $name {
			fn deserialize(reader: &mut R) -> Res<Self> {
				let mut bytes = [0u8; $n];
				reader.read(&mut bytes)?;
				Ok(Self(unsafe { std::mem::transmute(bytes) }))
			}
		}
		impl<W: std::io::Write> endio::Serialize<LE, W> for &$name {
			fn serialize(self, writer: &mut W) -> Res<()> {
				writer.write_all(&self.0)
			}
		}
	}
}

macro_rules! lu_wstr {
	($name:ident, $n:literal) => {
		// todo: runtime type invariants checks (valid ucs-2, null terminator)
		pub struct $name(pub [u16; $n]);

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
				write!(f, "{}", String::from(self))
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
			fn from(wstr: &$name) -> String {
				let terminator = wstr.0.iter().position(|&c| c == 0).unwrap();
				String::from_utf16(&wstr.0[..terminator]).unwrap()
			}
		}

		impl<R: std::io::Read> endio::Deserialize<LE, R> for $name {
			fn deserialize(reader: &mut R) -> Res<Self> {
				let mut bytes = [0u8; $n*2];
				reader.read(&mut bytes)?;
				Ok(Self(unsafe { std::mem::transmute(bytes) }))
			}
		}

		impl<W: std::io::Write> endio::Serialize<LE, W> for &$name {
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
lu_wstr!(LuWStr128, 128);
lu_wstr!(LuWStr256, 256);
