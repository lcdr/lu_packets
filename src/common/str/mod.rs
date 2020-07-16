mod fixed;
mod variable;

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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct AsciiChar(u8);
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Ucs2Char(u16);

impl AsciiChar {
	pub fn new(i: u8) -> Self {
		// todo: range check
		Self(i)
	}
}

impl Ucs2Char {
	pub fn new(i: u16) -> Self {
		// todo: range check
		Self(i)
	}
}

impl LuChar for AsciiChar {
	type Int = u8;
	type Error = AsciiError;
}

impl LuChar for Ucs2Char {
	type Int = u16;
	type Error = Ucs2Error;
}


type AbstractLuStr<C> = [C];

pub type LuStr = [AsciiChar];
pub type LuWStr = [Ucs2Char];

pub trait LuStrExt {
	type Char: LuChar;

	fn as_slice(&self) -> &[<Self::Char as LuChar>::Int];
	fn to_string(&self) -> String;

	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "{:?}", self.to_string())
	}
}

impl LuStrExt for LuStr {
	type Char = AsciiChar;

	fn as_slice(&self) -> &[<Self::Char as LuChar>::Int] {
		unsafe { &*(self as *const [Self::Char] as *const [<Self::Char as LuChar>::Int]) }
	}

	fn to_string(&self) -> String {
		std::str::from_utf8(self.as_slice()).unwrap().into()
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
}
