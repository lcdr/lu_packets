//! Shared types.
mod str;

use std::convert::{TryFrom, TryInto};
use std::io::{Read, Write};
use std::io::Result as Res;
use std::marker::PhantomData;

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};

pub use self::str::*;

/**
	Wraps a `Vec` with a length type so the vector can be (de-)serialized.

	Note: the length type is not checked and the `Vec` still uses `usize` internally. Handle with care.
*/
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LVec<T, L>(Vec<T>, PhantomData<L>);

impl<T, L> LVec<T, L> {
	pub fn new() -> Self {
		Self(Vec::new(), PhantomData)
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self(Vec::with_capacity(capacity), PhantomData)
	}

	pub fn inner(&self) -> &Vec<T> {
		&self.0
	}

	pub(crate) fn deser_content<R: Read>(reader: &mut R, str_len: L) -> Res<Self> where L: TryInto<usize> {
		let str_len = match str_len.try_into() {
			Ok(x) => x,
			_ => panic!(),
		};
		let mut string = Vec::<T>::with_capacity(str_len);
		unsafe {
			string.set_len(str_len);
			let mut ucs2_str_slice = std::slice::from_raw_parts_mut(string.as_mut_ptr() as *mut u8, str_len*std::mem::size_of::<T>());
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
		let u8_slice = unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.0.len()*std::mem::size_of::<T>()) };
		writer.write_all(u8_slice)
	}
}

impl<T, L, R: Read> Deserialize<LE, R> for LVec<T, L>
	where L: TryInto<usize> + Deserialize<LE, R> {

	fn deserialize(reader: &mut R) -> Res<Self> {
		let str_len: L = LERead::read(reader)?;
		Self::deser_content(reader, str_len)
	}
}

impl<'a, T, L, W: Write> Serialize<LE, W> for &'a LVec<T, L>
	where L: TryFrom<usize> + Serialize<LE, W> {

	fn serialize(self, writer: &mut W) -> Res<()> {
		self.ser_len(writer)?;
		self.ser_content(writer)
	}
}

impl<T, L> std::ops::Deref for LVec<T, L> {
	type Target = Vec<T>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T, L> std::ops::DerefMut for LVec<T, L> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T, L> From<Vec<T>> for LVec<T, L> {
	fn from(vec: Vec<T>) -> Self {
		Self(vec, PhantomData)
	}
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

pub type ObjId = u64;
pub const OBJID_EMPTY: u64 = 0;
