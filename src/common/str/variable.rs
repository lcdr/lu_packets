use std::convert::{TryFrom};
use std::marker::PhantomData;

use crate::common::LVec;
use super::{AsciiChar, AsciiError, LuStrExt, LuWStr, Ucs2Char, Ucs2Error};

pub type LuVarString<L> = LVec<AsciiChar, L>;
pub type LuVarWString<L> = LVec<Ucs2Char, L>;

impl<L> std::fmt::Debug for LuVarString<L> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		(&**self).fmt(f)
	}
}

impl<L> std::fmt::Debug for LuVarWString<L> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		(&**self).fmt(f)
	}
}

impl<L> From<&LuVarString<L>> for String {
	fn from(string: &LuVarString<L>) -> String {
		(&**string).to_string()
	}
}

impl<L> From<&LuVarWString<L>> for String {
	fn from(string: &LuVarWString<L>) -> String {
		(&**string).to_string()
	}
}

impl<L> TryFrom<&[u8]> for LuVarString<L> {
	type Error = AsciiError;

	fn try_from(string: &[u8]) -> Result<Self, Self::Error> {
		// todo: check for invalid character ranges for ascii
		Ok(Self(unsafe { (&*(string as *const [u8] as *const [AsciiChar])).into() }, PhantomData))
	}
}

impl<L> From<&LuWStr> for LuVarWString<L> {
	fn from(string: &LuWStr) -> Self {
		Self(string.into(), PhantomData)
	}
}

impl<L> TryFrom<&str> for LuVarWString<L> {
	type Error = Ucs2Error;

	fn try_from(string: &str) -> Result<Self, Self::Error> {
		let chars: Vec<u16> = string.encode_utf16().collect();
		// todo: check for invalid character ranges for ucs 2
		let chars = unsafe { std::mem::transmute(chars) };
		Ok(Self(chars, PhantomData))
	}
}
