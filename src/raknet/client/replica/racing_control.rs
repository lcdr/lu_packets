use std::io::{Read, Result as Res, Write};

use endio::{Deserialize, LE, Serialize};
use endio_bit::{BEBitReader, BEBitWriter};
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::{LuVarWString, LVec, ObjId};
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization, ReplicaD};
use super::scripted_activity::ActivityUserInfo;

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PreRacePlayerInfo {
	pub player_id: ObjId,
	pub vehicle_id: ObjId,
	pub starting_position: u32,
	pub is_ready: bool,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PostRacePlayerInfo {
	pub player_id: ObjId,
	pub current_rank: u32,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct RaceInfo {
	pub lap_count: u16,
	pub path_name: LuVarWString<u16>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct DuringRacePlayerInfo {
	pub player_id: ObjId,
	pub best_lap_time: f32,
	pub race_time: f32,
}

#[derive(BitVariantTests, Debug, PartialEq)]
pub struct RacingControlConstruction {
	pub activity_user_infos: Option<LVec<u32, ActivityUserInfo>>,
	pub expected_player_count: Option<u16>,
	pub pre_race_player_infos: Option<Vec<PreRacePlayerInfo>>,
	pub post_race_player_infos: Option<Vec<PostRacePlayerInfo>>,
	pub race_info: Option<RaceInfo>,
	pub during_race_player_infos: Option<Vec<DuringRacePlayerInfo>>,
}

impl<R: Read> Deserialize<LE, BEBitReader<R>> for RacingControlConstruction {
	fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self> {
		let activity_user_infos = ReplicaD::deserialize(reader)?;
		let expected_player_count = ReplicaD::deserialize(reader)?;
		let pre_race_player_infos = if reader.read_bit()? {
			let mut infos = vec![];
			while reader.read_bit()? {
				let info = ReplicaD::deserialize(reader)?;
				infos.push(info);
			}
			Some(infos)
		} else {
			None
		};
		let post_race_player_infos = if reader.read_bit()? {
			let mut infos = vec![];
			while reader.read_bit()? {
				let info = ReplicaD::deserialize(reader)?;
				infos.push(info);
			}
			Some(infos)
		} else {
			None
		};
		let race_info = ReplicaD::deserialize(reader)?;
		let during_race_player_infos = if reader.read_bit()? {
			let mut infos = vec![];
			while reader.read_bit()? {
				let info = ReplicaD::deserialize(reader)?;
				infos.push(info);
			}
			Some(infos)
		} else {
			None
		};
		Ok(Self { activity_user_infos, expected_player_count, pre_race_player_infos, post_race_player_infos, race_info, during_race_player_infos })
	}
}

impl<'a, W: Write> Serialize<LE, BEBitWriter<W>> for &'a RacingControlConstruction {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		crate::raknet::client::replica::ReplicaS::serialize(&self.activity_user_infos, writer)?;
		crate::raknet::client::replica::ReplicaS::serialize(&self.expected_player_count, writer)?;
		writer.write_bit(self.pre_race_player_infos.is_some())?;
		if let Some(infos) = &self.pre_race_player_infos {
			for info in infos {
				writer.write_bit(true)?;
				crate::raknet::client::replica::ReplicaS::serialize(info, writer)?;
			}
			writer.write_bit(false)?;
		}
		writer.write_bit(self.post_race_player_infos.is_some())?;
		if let Some(infos) = &self.post_race_player_infos {
			for info in infos {
				writer.write_bit(true)?;
				crate::raknet::client::replica::ReplicaS::serialize(info, writer)?;
			}
			writer.write_bit(false)?;
		}
		crate::raknet::client::replica::ReplicaS::serialize(&self.race_info, writer)?;
		writer.write_bit(self.during_race_player_infos.is_some())?;
		if let Some(infos) = &self.during_race_player_infos {
			for info in infos {
				writer.write_bit(true)?;
				crate::raknet::client::replica::ReplicaS::serialize(info, writer)?;
			}
			writer.write_bit(false)?;
		}
		Ok(())
	}
}

impl ComponentConstruction for RacingControlConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub type RacingControlSerialization = RacingControlConstruction;

impl ComponentSerialization for RacingControlSerialization {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub struct RacingControlProtocol;

impl ComponentProtocol for RacingControlProtocol {
	type Construction = RacingControlConstruction;
	type Serialization = RacingControlSerialization;
}
