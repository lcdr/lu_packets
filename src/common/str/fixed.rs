use std::convert::TryFrom;
use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LE, Serialize};

use super::{AbstractLuStr, AsciiChar, AsciiError, LuChar, LuStrExt, Ucs2Char, Ucs2Error};

// todo[const generics]: const generic strings
// todo: exclude the final null terminator from the array
macro_rules! abstract_lu_str {
	($name:ident, $c:ty, $null:expr, $n:literal) => {
		// todo: runtime type invariants checks (valid, null terminator)
		/// A string with a maximum length of $n.
		pub struct $name([$c; $n]);

		impl PartialEq for $name {
			fn eq(&self, other: &Self) -> bool {
				dbg!((&**self) == (&**other))
			}
		}

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
				(&**self).fmt(f)
			}
		}

		impl std::ops::Deref for $name {
			type Target = AbstractLuStr<$c>;

			#[inline]
			fn deref(&self) -> &Self::Target {
				let terminator = self.0.iter().position(|&c| c == $null).unwrap();
				&self.0[..terminator]
			}
		}

		impl std::ops::DerefMut for $name {
			#[inline]
			fn deref_mut(&mut self) -> &mut Self::Target {
				let terminator = self.0.iter().position(|&c| c == $null).unwrap();
				&mut self.0[..terminator]
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
		abstract_lu_str!($name, AsciiChar, AsciiChar(0), $n);

		impl TryFrom<&[u8]> for $name {
			type Error = AsciiError;

			fn try_from(string: &[u8]) -> Result<Self, Self::Error> {
				if string.len() >= $n {
					// actually length error but whatever
					return Err(AsciiError);
				}
				let mut bytes = [0u8; $n];
				// todo: ascii range check
				for (i, chr) in string.iter().enumerate() {
					bytes[i] = *chr;
				}
				let bytes = unsafe { std::mem::transmute(bytes) };
				Ok(Self(bytes))
			}
		}

		impl<const N: usize> TryFrom<&[u8; N]> for $name {
			type Error = AsciiError;

			fn try_from(string: &[u8; N]) -> Result<Self, Self::Error> {
				Self::try_from(&string[..])
			}
		}
	}
}

macro_rules! lu_wstr {
	($name:ident, $n:literal) => {
		abstract_lu_str!($name, Ucs2Char, Ucs2Char(0), $n);

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
				String::from_utf16(unsafe {&*(&**wstr as *const [Ucs2Char] as *const [<Ucs2Char as LuChar>::Int])}).unwrap()
			}
		}
	}
}

lu_str!(LuString33, 33);
lu_wstr!(LuWString32, 32);
lu_wstr!(LuWString33, 33);
lu_wstr!(LuWString41, 41);
lu_wstr!(LuWString42, 42);
lu_wstr!(LuWString50, 50);
lu_wstr!(LuWString128, 128);
lu_wstr!(LuWString256, 256);
lu_wstr!(LuWString400, 400);
