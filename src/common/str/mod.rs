mod fixed;
mod variable;

use endio::{Deserialize, Serialize};

pub use self::fixed::*;
pub use self::variable::*;

pub trait LuChar {
	type Int;
	type Error;
}

#[derive(Debug)]
pub struct AsciiError;

#[derive(Debug)]
pub struct Ucs2Error;

#[derive(Clone, Copy, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AsciiChar(u8);
#[derive(Clone, Copy, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Ucs2Char(u16);

impl LuChar for AsciiChar {
	type Int = u8;
	type Error = AsciiError;
}

impl LuChar for Ucs2Char {
	type Int = u16;
	type Error = Ucs2Error;
}

impl From<u8> for AsciiChar {
	fn from(byte: u8) -> Self {
		// todo: range check
		Self(byte)
	}
}

impl From<u8> for Ucs2Char {
	fn from(byte: u8) -> Self {
		// todo: range check
		Self(byte as u16)
	}
}

type AbstractLuStr<C> = [C];

pub type LuStr = AbstractLuStr<AsciiChar>;
pub type LuWStr = AbstractLuStr<Ucs2Char>;

pub trait LuStrExt {
	type Char: LuChar;

	fn from_slice(slice: &[<Self::Char as LuChar>::Int]) -> &[Self::Char] {
		unsafe { &*(slice as *const [<Self::Char as LuChar>::Int] as *const [Self::Char]) }
	}

	fn as_slice(&self) -> &[<Self::Char as LuChar>::Int];
	fn to_string(&self) -> String;

	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>;
}

impl LuStrExt for LuStr {
	type Char = AsciiChar;

	fn as_slice(&self) -> &[<Self::Char as LuChar>::Int] {
		unsafe { &*(self as *const [Self::Char] as *const [<Self::Char as LuChar>::Int]) }
	}

	fn to_string(&self) -> String {
		std::str::from_utf8(self.as_slice()).unwrap().into()
	}

	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "b{:?}", self.to_string())
	}
}

impl LuStrExt for LuWStr {
	type Char = Ucs2Char;

	fn as_slice(&self) -> &[<Self::Char as LuChar>::Int] {
		unsafe { &*(self as *const [Self::Char] as *const [<Self::Char as LuChar>::Int]) }
	}

	fn to_string(&self) -> String {
		String::from_utf16(self.as_slice()).unwrap()
	}

	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "{:?}", self.to_string())
	}
}
