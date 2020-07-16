use std::convert::TryFrom;
use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LE, Serialize};

use super::{AbstractLuStr, AsciiChar, AsciiError, LuChar, LuStrExt, Ucs2Char, Ucs2Error};

// todo[const generics]: const generic strings
// todo: exclude the final null terminator from the array
macro_rules! abstract_lu_str {
	($name:ident, $c:ty, $n:literal) => {
		// todo: runtime type invariants checks (valid, null terminator)
		/// A string with a maximum length of $n.
		pub struct $name([$c; $n]);

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
				(&**self).fmt(f)
			}
		}

		impl<R: Read> Deserialize<LE, R> for $name {
			fn deserialize(reader: &mut R) -> Res<Self> {
				let mut bytes = [0u8; $n*std::mem::size_of::<$c>()];
				reader.read(&mut bytes)?;
				Ok(Self(unsafe { std::mem::transmute(bytes) }))
			}
		}

		impl<W: Write> Serialize<LE, W> for &$name {
			fn serialize(self, writer: &mut W) -> Res<()> {
				let x: [u8; $n*std::mem::size_of::<$c>()] = unsafe { std::mem::transmute(self.0) };
				writer.write_all(&x)
			}
		}
	}
}

macro_rules! lu_str {
	($name:ident, $n:literal) => {
		abstract_lu_str!($name, AsciiChar, $n);

		impl std::ops::Deref for $name {
			type Target = AbstractLuStr<AsciiChar>;

			#[inline]
			fn deref(&self) -> &Self::Target {
				let terminator = self.0.iter().position(|&c| c == AsciiChar(0)).unwrap();
				&self.0[..terminator]
			}
		}

		impl TryFrom<&str> for $name {
			type Error = AsciiError;

			fn try_from(string: &str) -> Result<Self, Self::Error> {
				let mut bytes = [0u8; $n];
				// todo: ascii range check
				for (i, chr) in string.bytes().take($n-1).enumerate() {
					bytes[i] = chr;
				}
				let bytes = unsafe { std::mem::transmute(bytes) };
				Ok(Self(bytes))
			}
		}
	}
}

macro_rules! lu_wstr {
	($name:ident, $n:literal) => {
		abstract_lu_str!($name, Ucs2Char, $n);

		impl std::ops::Deref for $name {
			type Target = AbstractLuStr<Ucs2Char>;

			#[inline]
			fn deref(&self) -> &Self::Target {
				let terminator = self.0.iter().position(|&c| c == Ucs2Char(0)).unwrap();
				&self.0[..terminator]
			}
		}

		impl TryFrom<&str> for $name {
			type Error = Ucs2Error;

			fn try_from(string: &str) -> Result<Self, Self::Error> {
				let mut bytes = [0u16; $n];
				for (i, chr) in string.encode_utf16().take($n-1).enumerate() {
					bytes[i] = chr;
				}
				let bytes = unsafe { std::mem::transmute(bytes) };
				Ok(Self(bytes))
			}
		}

		impl From<&$name> for String {
			fn from(wstr: &$name) -> Self {
				let terminator = wstr.0.iter().position(|&c| c == Ucs2Char(0)).unwrap();
				String::from_utf16(unsafe {&*(&wstr.0[..terminator] as *const [Ucs2Char] as *const [<Ucs2Char as LuChar>::Int])}).unwrap()
			}
		}
	}
}

lu_str!(LuString33, 33);
lu_wstr!(LuWString33, 33);
lu_wstr!(LuWString41, 41);
lu_wstr!(LuWString42, 42);
lu_wstr!(LuWString128, 128);
lu_wstr!(LuWString256, 256);
