//! Shared types.
mod str;

use std::convert::{TryFrom, TryInto};
use std::fmt::{Formatter, Debug};
use std::io::{Read, Write};
use std::io::Result as Res;
use std::marker::PhantomData;

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};

pub use self::str::*;

/**
	Wraps a `Vec` with a length type so the vector can be (de-)serialized.

	Note: the length type is not checked and the `Vec` still uses `usize` internally. Handle with care.
*/
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LVec<L, T>(Vec<T>, PhantomData<L>);

impl<L, T> LVec<L, T> {
	pub fn new() -> Self {
		Self(Vec::new(), PhantomData)
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self(Vec::with_capacity(capacity), PhantomData)
	}

	pub fn inner(&self) -> &Vec<T> {
		&self.0
	}

	pub(crate) fn deser_content<R: Read>(reader: &mut R, len: L) -> Res<Self> where L: TryInto<usize>, T: Deserialize<LE, R> {
		let len = match len.try_into() {
			Ok(x) => x,
			_ => panic!(),
		};
		let mut vec = Vec::<T>::with_capacity(len);
		for _ in 0..len {
			vec.push(LERead::read(reader)?);
		}
		Ok(Self(vec, PhantomData))
	}

	pub(crate) fn ser_len<W: LEWrite>(&self, writer: &mut W) -> Res<()> where L: TryFrom<usize> + Serialize<LE, W> {
		let len = self.0.len();
		let l_len = match L::try_from(len) {
			Ok(x) => x,
			_ => panic!(),
		};
		writer.write(l_len)
	}

	pub(crate) fn ser_content<W: Write>(&self, writer: &mut W) -> Res<()> where for<'a> &'a T: Serialize<LE, W> {
		for e in &self.0 {
			LEWrite::write(writer, e)?;
		}
		Ok(())
	}
}

impl<L, T: Debug> Debug for LVec<L, T> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl<L, T, R: Read> Deserialize<LE, R> for LVec<L, T>
	where L: TryInto<usize> + Deserialize<LE, R>,
	      T: Deserialize<LE, R>	{

	fn deserialize(reader: &mut R) -> Res<Self> {
		let len: L = LERead::read(reader)?;
		Self::deser_content(reader, len)
	}
}

impl<'a, L, T, W: Write> Serialize<LE, W> for &'a LVec<L, T>
	where L: TryFrom<usize> + Serialize<LE, W>,
	  for<'b> &'b T: Serialize<LE, W> {

	fn serialize(self, writer: &mut W) -> Res<()> {
		self.ser_len(writer)?;
		self.ser_content(writer)
	}
}

impl<L, T> std::ops::Deref for LVec<L, T> {
	type Target = Vec<T>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<L, T> std::ops::DerefMut for LVec<L, T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<L, T> From<Vec<T>> for LVec<L, T> {
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
