use std::io::{Read, Result as Res, Write};

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};
use endio_bit::{BEBitReader, BEBitWriter};
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::LVec;
use super::ComponentConstruction;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StatusImmunityInfo {
	pub immune_to_basic_attack: u32,
	pub immune_to_damage_over_time: u32,
	pub immune_to_knockback: u32,
	pub immune_to_interrupt: u32,
	pub immune_to_speed: u32,
	pub immune_to_imagination_gain: u32,
	pub immune_to_imagination_loss: u32,
	pub immune_to_quickbuild_interrupt: u32,
	pub immune_to_pull_to_point: u32,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct SmashableInfo {
	pub is_module_assembly: bool,
	pub explode_factor: Option<f32>,
}

// so close to being able to do serialization automatically...if not for the irregularity with `smashable_info`...
#[derive(Debug, PartialEq)]
pub struct StatsInfo {
	pub cur_health: u32,
	pub max_health: f32,
	pub cur_armor: u32,
	pub max_armor: f32,
	pub cur_imag: u32,
	pub max_imag: f32,
	pub damage_absorption_points: u32,
	pub immunity: bool,
	pub is_gm_immune: bool,
	pub is_shielded: bool,
	pub actual_max_health: f32,
	pub actual_max_armor: f32,
	pub actual_max_imag: f32,
	pub factions: LVec<u32, i32>,
	pub is_dead: bool,
	pub is_smashed: bool,
	pub smashable_info: Option<SmashableInfo>,
}

impl<R: Read> Deserialize<LE, BEBitReader<R>> for StatsInfo {
	fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self> {
		let cur_health = LERead::read(reader)?;
		let max_health = LERead::read(reader)?;
		let cur_armor  = LERead::read(reader)?;
		let max_armor  = LERead::read(reader)?;
		let cur_imag   = LERead::read(reader)?;
		let max_imag   = LERead::read(reader)?;
		let damage_absorption_points = LERead::read(reader)?;
		let immunity     = reader.read_bit()?;
		let is_gm_immune = reader.read_bit()?;
		let is_shielded  = reader.read_bit()?;
		let actual_max_health = LERead::read(reader)?;
		let actual_max_armor  = LERead::read(reader)?;
		let actual_max_imag   = LERead::read(reader)?;
		let factions          = LERead::read(reader)?;
		let is_smashable = reader.read_bit()?;
		let is_dead      = reader.read_bit()?;
		let is_smashed   = reader.read_bit()?;
		let smashable_info = if is_smashable {
			Some(LERead::read(reader)?)
		} else {
			None
		};
		Ok(Self {
				cur_health,
				max_health,
				cur_armor,
				max_armor,
				cur_imag,
				max_imag,
				damage_absorption_points,
				immunity,
				is_gm_immune,
				is_shielded,
				actual_max_health,
				actual_max_armor,
				actual_max_imag,
				factions,
				is_dead,
				is_smashed,
				smashable_info,
		})
	}
}

impl<'a, W: Write> Serialize<LE, BEBitWriter<W>> for &'a StatsInfo {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		LEWrite::write(writer, self.cur_health)?;
		LEWrite::write(writer, self.max_health)?;
		LEWrite::write(writer, self.cur_armor)?;
		LEWrite::write(writer, self.max_armor)?;
		LEWrite::write(writer, self.cur_imag)?;
		LEWrite::write(writer, self.max_imag)?;
		LEWrite::write(writer, self.damage_absorption_points)?;
		writer.write_bit(self.immunity)?;
		writer.write_bit(self.is_gm_immune)?;
		writer.write_bit(self.is_shielded)?;
		LEWrite::write(writer, self.actual_max_health)?;
		LEWrite::write(writer, self.actual_max_armor)?;
		LEWrite::write(writer, self.actual_max_imag)?;
		LEWrite::write(writer, &self.factions)?;
		writer.write_bit(self.smashable_info.is_some())?;
		writer.write_bit(self.is_dead)?;
		writer.write_bit(self.is_smashed)?;
		if let Some(x) = &self.smashable_info {
			LEWrite::write(writer, x)?;
		}
		Ok(())
	}
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct DestroyableConstruction {
	pub status_immunity_info: Option<StatusImmunityInfo>,
	pub stats_info: Option<StatsInfo>,
	pub is_on_a_threat_list: Option<bool>,
}


impl ComponentConstruction for DestroyableConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
