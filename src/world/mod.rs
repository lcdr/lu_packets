pub mod client;
pub mod server;

use std::cmp::PartialEq;
use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use lu_packets_derive::GmParam;

use crate::common::{LuVarStr, LuVarWStr};

type GmString = LuVarStr<u32>;
type GmWString = LuVarWStr<u32>;

type Lot = u32;
const LOT_NULL: Lot = -1i32 as Lot;

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneId {
	pub map_id: u16,
	pub instance_id: u16,
	pub clone_id: u32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector3 {
	const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
pub struct Quaternion {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Quaternion {
	const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
}

trait GmParam: Sized {
	fn deserialize<R: Read>(reader: &mut R) -> Res<Self>;
	fn serialize<W: Write>(&self, writer: &mut W) -> Res<()>;
}

macro_rules! impl_gm {
	($typ:ty) => {
		impl GmParam for $typ {
			fn deserialize<R: Read>(reader: &mut R) -> Res<Self> {
				LERead::read(reader)
			}

			fn serialize<W: Write>(&self, writer: &mut W) -> Res<()> {
				LEWrite::write(writer, self)
			}
		}
	}
}

impl_gm!(u32);
impl_gm!(u64);
impl_gm!(i32);
impl_gm!(f32);
impl_gm!(GmString);
impl_gm!(GmWString);

impl GmParam for Vec<u8> {
	fn deserialize<R: Read>(reader: &mut R) -> Res<Self> {
		let str_len: u32 = LERead::read(reader)?;
		let str_len = str_len as usize;
		let mut vec = Vec::with_capacity(str_len);
		unsafe { vec.set_len(str_len); }
		Read::read(reader, &mut vec)?;
		Ok(vec)
	}

	fn serialize<W: Write>(&self, writer: &mut W) -> Res<()> {
		LEWrite::write(writer, self.len() as u32)?;
		Write::write_all(writer, self)
	}
}
