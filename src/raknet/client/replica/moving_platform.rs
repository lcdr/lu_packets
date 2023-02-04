use std::io::{Read, Result as Res, Write};

use endio::{Deserialize, LE, Serialize};
use endio_bit::{BEBitReader, BEBitWriter};
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::LuVarWString;
use crate::world::Vector3;
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization, ReplicaD};
use super::simple_physics::PositionRotationInfo;

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PlatformMoverInfo {
	/// todo: bitfield
	pub state: u32,
	/// todo: u32 with special case for -1
	pub desired_waypoint_index: i32,
	pub stop_at_desired_waypoint: bool,
	pub is_in_reverse: bool,
	pub percent_to_next_waypoint: f32,
	/// not completely sure what this position is
	pub position: Vector3,
	pub current_waypoint_index: u32,
	pub next_waypoint_index: u32,
	pub idle_time_elapsed: f32,
	pub move_time_elapsed: f32,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PlatformSimpleMoverExtraInfo {
	/// todo: bitfield
	pub state: u32,
	pub current_waypoint_index: u32,
	pub is_in_reverse: bool,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PlatformSimpleMoverInfo {
	pub start_point_position_rotation_info: Option<Option<PositionRotationInfo>>,
	pub extra_info: Option<PlatformSimpleMoverExtraInfo>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
#[repr(u32)]
pub enum PlatformSubcomponentInfo {
	Mover(Option<PlatformMoverInfo>) = 4,
	SimpleMover(PlatformSimpleMoverInfo) = 5,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PlatformPathInfo {
	pub path_name: LuVarWString<u16>,
	pub starting_waypoint: u32,
	pub is_in_reverse: bool,
}

#[derive(BitVariantTests, Debug, PartialEq)]
pub struct MovingPlatformConstruction {
	pub path_info: Option<PlatformPathInfo>,
	pub subcomponent_infos: Option<Vec<PlatformSubcomponentInfo>>,
}

impl<R: Read> Deserialize<LE, BEBitReader<R>> for MovingPlatformConstruction {
	fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self> {
		let has_subcomponent_infos = reader.read_bit()?;
		let flag = reader.read_bit()?;
		let path_info = if flag { ReplicaD::deserialize(reader)? } else { None };
		let subcomponent_infos = if has_subcomponent_infos {
			let mut infos = vec![];
			while reader.read_bit()? {
				let subcomp = ReplicaD::deserialize(reader)?;
				infos.push(subcomp);
			}
			Some(infos)
		} else {
			None
		};
		Ok(Self { path_info, subcomponent_infos })
	}
}

impl<'a, W: Write> Serialize<LE, BEBitWriter<W>> for &'a MovingPlatformConstruction {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		writer.write_bit(self.subcomponent_infos.is_some())?;
		if let Some(path_info) = &self.path_info {
			if !path_info.path_name.is_empty() {
				writer.write_bit(true)?;
				writer.write_bit(true)?;
				crate::raknet::client::replica::ReplicaS::serialize(path_info, writer)?;
			} else {
				writer.write_bit(false)?;
			}
		} else {
			writer.write_bit(false)?;
		}
		if let Some(subcomponent_infos) = &self.subcomponent_infos {
			for sci in subcomponent_infos {
				writer.write_bit(true)?;
				crate::raknet::client::replica::ReplicaS::serialize(sci, writer)?;
			}
			writer.write_bit(false)?;
		}
		Ok(())
	}
}

impl ComponentConstruction for MovingPlatformConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub type MovingPlatformSerialization = MovingPlatformConstruction;

impl ComponentSerialization for MovingPlatformSerialization {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub struct MovingPlatformProtocol;

impl ComponentProtocol for MovingPlatformProtocol {
	type Construction = MovingPlatformConstruction;
	type Serialization = MovingPlatformSerialization;
}
