use std::convert::{TryFrom, TryInto};
use std::io::{Read, Write};
use std::io::Result as Res;
use std::marker::PhantomData;

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};

use super::{AsciiChar, AsciiError, LuStrExt, LuWStr, Ucs2Char, Ucs2Error};

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AbstractLuVarString<C, L>(Vec<C>, PhantomData<L>);

pub type LuVarString<L> = AbstractLuVarString<AsciiChar, L>;
pub type LuVarWString<L> = AbstractLuVarString<Ucs2Char, L>;

impl<C, L> AbstractLuVarString<C, L> {
	pub fn new(vec: Vec<C>) -> Self {
		Self(vec, PhantomData)
	}

	pub(crate) fn deser_content<R: Read>(reader: &mut R, str_len: L) -> Res<Self> where L: TryInto<usize> {
		let str_len = match str_len.try_into() {
			Ok(x) => x,
			_ => panic!(),
		};
		let mut string = Vec::<C>::with_capacity(str_len);
		unsafe {
			string.set_len(str_len);
			let mut ucs2_str_slice = std::slice::from_raw_parts_mut(string.as_mut_ptr() as *mut u8, str_len*std::mem::size_of::<C>());
			Read::read(reader, &mut ucs2_str_slice)?;
		}
		Ok(Self(string, PhantomData))
	}

	pub(crate) fn ser_len<W: LEWrite>(&self, writer: &mut W) -> Res<()> where L: TryFrom<usize> + Serialize<LE, W> {
		let str_len = self.0.len();
		let l_len = match L::try_from(str_len) {
			Ok(x) => x,
			_ => panic!(),
		};
		writer.write(l_len)
	}

	pub(crate) fn ser_content<W: Write>(&self, writer: &mut W) -> Res<()> {
		let u8_slice = unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.0.len()*std::mem::size_of::<C>()) };
		writer.write_all(u8_slice)
	}
}

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

impl<C, L, R: Read> Deserialize<LE, R> for AbstractLuVarString<C, L>
	where L: TryInto<usize> + Deserialize<LE, R> {

	fn deserialize(reader: &mut R) -> Res<Self> {
		let str_len: L = LERead::read(reader)?;
		Self::deser_content(reader, str_len)
	}
}

impl<'a, C, L, W: Write> Serialize<LE, W> for &'a AbstractLuVarString<C, L>
	where L: TryFrom<usize> + Serialize<LE, W> {

	fn serialize(self, writer: &mut W) -> Res<()> {
		self.ser_len(writer)?;
		self.ser_content(writer)
	}
}

impl<C, L> std::ops::Deref for AbstractLuVarString<C, L> {
	type Target = Vec<C>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<C, L> std::ops::DerefMut for AbstractLuVarString<C, L> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
