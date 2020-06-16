pub mod client;
pub mod server;

use std::io::{Error, ErrorKind::InvalidData, Read};
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use lu_packets_derive::GmDeserialize;

type Lot = u32;
const LOT_NULL: Lot = -1i32 as Lot;

#[derive(Debug, Deserialize, GmDeserialize)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector3 {
	const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
}

#[derive(Debug, Deserialize, GmDeserialize)]
pub struct Quaternion {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Quaternion {
	const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
}

trait GmDeserialize<R: Read>: Sized {
	fn deserialize(reader: &mut R) -> Res<Self>;
}

macro_rules! impl_gm {
	($typ:ty) => {
		impl<R: Read> GmDeserialize<R> for $typ {
			fn deserialize(reader: &mut R) -> Res<Self> {
				LERead::read(reader)
			}
		}
	}
}

impl_gm!(u32);
impl_gm!(u64);
impl_gm!(i32);
impl_gm!(f32);

impl<R: Read> GmDeserialize<R> for Vec<u8> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let str_len: u32 = LERead::read(reader)?;
		let str_len = str_len as usize;
		if str_len == 0 {
			return Ok(Vec::new());
		}
		let mut vec = Vec::with_capacity(str_len);
		unsafe { vec.set_len(str_len); }
		Read::read(reader, &mut vec)?;
		Ok(vec)
	}
}

impl<R: Read> GmDeserialize<R> for String {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let str_len: u32 = LERead::read(reader)?;
		let str_len = str_len as usize;
		if str_len == 0 {
			return Ok(String::new());
		}
		let mut ucs2_str = Vec::<u16>::with_capacity(str_len);
		unsafe {
			ucs2_str.set_len(str_len);
			let mut ucs2_str_slice = std::slice::from_raw_parts_mut(ucs2_str.as_mut_ptr() as *mut u8, str_len*2);
			Read::read(reader, &mut ucs2_str_slice)?;
		}
		String::from_utf16(&ucs2_str[..]).map_err(|_| Error::new(InvalidData, "invalid utf16 str"))
	}
}
