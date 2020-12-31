//! World messages.
pub mod client;
pub mod gm;
pub mod amf3;
mod lnv;
pub mod server;

use std::cmp::PartialEq;

use endio::{Deserialize, Serialize};
use lu_packets_derive::GmParam;
pub use lnv::*;

pub type Lot = u32;
const LOT_NULL: Lot = -1i32 as Lot;

// todo: better modeling with NonNull and Option
type MapId = u16;
const MAP_ID_INVALID: MapId = 0;

type CloneId = u32;
const CLONE_ID_INVALID: CloneId = 0;

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
pub struct ZoneId {
	pub map_id: MapId,
	pub instance_id: u16,
	pub clone_id: CloneId,
}

impl ZoneId {
	const INVALID: Self = Self { map_id: 0, instance_id: 0, clone_id: 0 };
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector3 {
	pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
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
