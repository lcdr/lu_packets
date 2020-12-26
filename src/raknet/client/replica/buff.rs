use std::io::{Read, Result as Res, Write};

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};
use endio_bit::{BEBitReader, BEBitWriter};
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::{LVec, ObjId};
use super::{ReplicaD, ComponentConstruction};

// so close to being able to do serialization automatically...if not for the irregularity with `added_by_teammate`...
#[derive(Debug, PartialEq)]
pub struct BuffInfo {
	pub buff_id: u32,
	pub time_left: Option<u32>,
	pub cancel_on_death: bool,
	pub cancel_on_zone: bool,
	pub cancel_on_damaged: bool,
	pub cancel_on_remove_buff: bool,
	pub cancel_on_ui: bool,
	pub cancel_on_logout: bool,
	pub cancel_on_unequip: bool,
	pub cancel_on_damage_absorb_ran_out: bool,
	pub added_by_teammate: Option<ObjId>,
	pub apply_on_teammates: bool,
	pub ref_count: i32,
}

impl<R: Read> Deserialize<LE, BEBitReader<R>> for BuffInfo {
	fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self> {
		let buff_id = LERead::read(reader)?;
		let time_left = ReplicaD::deserialize(reader)?;
		let cancel_on_death = reader.read_bit()?;
		let cancel_on_zone = reader.read_bit()?;
		let cancel_on_damaged = reader.read_bit()?;
		let cancel_on_remove_buff = reader.read_bit()?;
		let cancel_on_ui = reader.read_bit()?;
		let cancel_on_logout = reader.read_bit()?;
		let cancel_on_unequip = reader.read_bit()?;
		let cancel_on_damage_absorb_ran_out = reader.read_bit()?;
		let added_by_teammate = reader.read_bit()?;
		let apply_on_teammates = reader.read_bit()?;
		let added_by_teammate = if added_by_teammate {
			Some(LERead::read(reader)?)
		} else {
			None
		};
		let ref_count = LERead::read(reader)?;
		Ok(Self {
				buff_id,
				time_left,
				cancel_on_death,
				cancel_on_zone,
				cancel_on_damaged,
				cancel_on_remove_buff,
				cancel_on_ui,
				cancel_on_logout,
				cancel_on_unequip,
				cancel_on_damage_absorb_ran_out,
				added_by_teammate,
				apply_on_teammates,
				ref_count,
		})
	}
}
impl<'a, W: Write> Serialize<LE, BEBitWriter<W>> for &'a BuffInfo {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		LEWrite::write(writer, self.buff_id)?;
		crate::raknet::client::replica::ReplicaS::serialize(&self.time_left, writer)?;
		writer.write_bit(self.cancel_on_death)?;
		writer.write_bit(self.cancel_on_zone)?;
		writer.write_bit(self.cancel_on_damaged)?;
		writer.write_bit(self.cancel_on_remove_buff)?;
		writer.write_bit(self.cancel_on_ui)?;
		writer.write_bit(self.cancel_on_logout)?;
		writer.write_bit(self.cancel_on_unequip)?;
		writer.write_bit(self.cancel_on_damage_absorb_ran_out)?;
		writer.write_bit(self.added_by_teammate.is_some())?;
		writer.write_bit(self.apply_on_teammates)?;
		if let Some(x) = self.added_by_teammate {
			LEWrite::write(writer, x)?;
		}
		LEWrite::write(writer, self.ref_count)?;
		Ok(())
	}
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct BuffConstruction {
	pub buffs: Option<LVec<u32, BuffInfo>>,
	pub immunities: Option<LVec<u32, BuffInfo>>,
}

impl ComponentConstruction for BuffConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
