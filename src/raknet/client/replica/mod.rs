pub mod achievement_vendor;
pub mod base_combat_ai;
pub mod bbb;
pub mod bouncer;
pub mod buff;
pub mod character;
pub mod collectible;
pub mod controllable_physics;
pub mod donation_vendor;
pub mod destroyable;
pub mod fx;
pub mod inventory;
pub mod item;
pub mod level_progression;
pub mod lup_exhibit;
pub mod mutable_model_behavior;
pub mod module_assembly;
pub mod moving_platform;
pub mod phantom_physics;
pub mod player_forced_movement;
pub mod pet;
pub mod possessable;
pub mod possession_control;
pub mod quickbuild;
pub mod racing_control;
pub mod rigid_body_phantom_physics;
pub mod script;
pub mod scripted_activity;
pub mod shooting_gallery;
pub mod simple_physics;
pub mod skill;
pub mod switch;
pub mod vehicle_physics;
pub mod vendor;

use std::fmt::Debug;
use std::io::{Read, Result as Res, Write};

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};
use endio_bit::{BEBitReader, BEBitWriter};
use lu_packets_derive::ReplicaSerde;

use crate::common::{ObjId, LuVarWString, LVec};
use crate::world::{Lot, LuNameValue};

trait ReplicaD<R: Read>: Sized {
	fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self>;
}

trait ReplicaS<W: Write> {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()>;
}

impl<R: Read, T: Deserialize<LE, BEBitReader<R>>> ReplicaD<R> for T {
	default fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self> {
		Deserialize::deserialize(reader)
	}
}

impl<'a, W: Write, T> ReplicaS<W> for &'a T where &'a T: Serialize<LE, BEBitWriter<W>> {
	default fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		Serialize::serialize(self, writer)
	}
}

impl<R: Read> ReplicaD<R> for bool {
	fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self> {
		reader.read_bit()
	}
}

impl<'a, W: Write> ReplicaS<W> for &'a bool {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		writer.write_bit(*self)
	}
}

impl<R: Read, T: ReplicaD<R>+Deserialize<LE, BEBitReader<R>>> ReplicaD<R> for Option<T> {
	fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self> {
		let bit = reader.read_bit()?;
		Ok(if !bit {
			None
		} else {
			Some(ReplicaD::deserialize(reader)?)
		})
	}
}

impl<W: Write, T> ReplicaS<W> for &Option<T> where for<'a> &'a T: ReplicaS<W>+Serialize<LE, BEBitWriter<W>> {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		writer.write_bit(self.is_some())?;
		if let Some(x) = self {
			ReplicaS::serialize(x, writer)?;
		}
		Ok(())
	}
}

pub trait ComponentConstruction: Debug {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()>;
}

pub trait ComponentSerialization: Debug {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()>;
}

pub trait ComponentProtocol {
	type Construction: ComponentConstruction;
	type Serialization: ComponentSerialization;
}

pub trait ReplicaContext {
	fn get_comp_constructions<R: Read>(&mut self, network_id: u16, lot: Lot, config: &Option<LuNameValue>) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>>;
	fn get_comp_serializations<R: Read>(&mut self, network_id: u16) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentSerialization>>>;
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ParentInfo {
	pub parent_id: ObjId,
	pub update_position_with_parent: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ChildInfo {
	pub child_ids: LVec<u16, ObjId>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ParentChildInfo {
	pub parent_info: Option<ParentInfo>,
	pub child_info: Option<ChildInfo>,
}

#[derive(Debug)]
pub struct ReplicaConstruction {
	pub network_id: u16,
	pub object_id: ObjId,
	pub lot: Lot,
	pub name: LuVarWString<u8>,
	pub time_since_created_on_server: u32,
	pub config: Option<LuNameValue>,
	pub is_trigger: bool,
	pub spawner_id: Option<ObjId>,
	pub spawner_node_id: Option<i32>,
	pub scale: Option<f32>,
	pub world_state: Option<u8>, // todo: type
	pub gm_level: Option<u8>, // todo: type
	pub parent_child_info: Option<ParentChildInfo>,
	pub components: Vec<Box<dyn ComponentConstruction>>,
}

impl PartialEq<ReplicaConstruction> for ReplicaConstruction {
	fn eq(&self, rhs: &ReplicaConstruction) -> bool {
		// hacky but i don't know a better way
		format!("{:?}", self) == format!("{:?}", rhs)
	}
}

impl<R: Read+ReplicaContext> Deserialize<LE, R> for ReplicaConstruction {
	#[rustfmt::skip]
	fn deserialize(reader: &mut R) -> Res<Self> {
		let mut bit_reader = BEBitReader::new(reader);
		let bit = bit_reader.read_bit()?;
		assert_eq!(bit, true);
		let network_id = LERead::read(&mut bit_reader)?;
		let object_id  = LERead::read(&mut bit_reader)?;
		let lot        = LERead::read(&mut bit_reader)?;
		let name       = LERead::read(&mut bit_reader)?;
		let time_since_created_on_server = LERead::read(&mut bit_reader)?;
		let config = ReplicaD::deserialize(&mut bit_reader)?;
		let is_trigger = bit_reader.read_bit()?;
		let spawner_id        = ReplicaD::deserialize(&mut bit_reader)?;
		let spawner_node_id   = ReplicaD::deserialize(&mut bit_reader)?;
		let scale             = ReplicaD::deserialize(&mut bit_reader)?;
		let world_state       = ReplicaD::deserialize(&mut bit_reader)?;
		let gm_level          = ReplicaD::deserialize(&mut bit_reader)?;
		let parent_child_info = ReplicaD::deserialize(&mut bit_reader)?;
		let mut components = vec![];
		for new in unsafe {bit_reader.get_mut_unchecked()}.get_comp_constructions(network_id, lot, &config) {
			components.push(new(&mut bit_reader)?);
		}

		Ok(Self {
			network_id,
			object_id,
			lot,
			name,
			time_since_created_on_server,
			config,
			is_trigger,
			spawner_id,
			spawner_node_id,
			scale,
			world_state,
			gm_level,
			parent_child_info,
			components,
		})
	}
}

impl<'a, W: Write> Serialize<LE, W> for &'a ReplicaConstruction {
	fn serialize(self, writer: &mut W) -> Res<()> {
		let mut bit_writer = BEBitWriter::new(vec![]);
		bit_writer.write_bit(true)?;
		LEWrite::write(&mut bit_writer, self.network_id)?;
		LEWrite::write(&mut bit_writer, self.object_id)?;
		LEWrite::write(&mut bit_writer, self.lot)?;
		LEWrite::write(&mut bit_writer, &self.name)?;
		LEWrite::write(&mut bit_writer, self.time_since_created_on_server)?;
		ReplicaS::serialize(&self.config, &mut bit_writer)?;
		bit_writer.write_bit(self.is_trigger)?;
		ReplicaS::serialize(&self.spawner_id, &mut bit_writer)?;
		ReplicaS::serialize(&self.spawner_node_id, &mut bit_writer)?;
		ReplicaS::serialize(&self.scale, &mut bit_writer)?;
		ReplicaS::serialize(&self.world_state, &mut bit_writer)?;
		ReplicaS::serialize(&self.gm_level, &mut bit_writer)?;
		ReplicaS::serialize(&self.parent_child_info, &mut bit_writer)?;

		for comp in &self.components {
			comp.ser(&mut bit_writer)?;
		}
		bit_writer.flush()?;
		LEWrite::write(writer, bit_writer.get_ref())?;
		Ok(())
	}
}

#[derive(Debug)]
pub struct ReplicaSerialization {
	pub network_id: u16,
	pub parent_child_info: Option<ParentChildInfo>,
	pub components: Vec<Box<dyn ComponentSerialization>>,
}

impl PartialEq<ReplicaSerialization> for ReplicaSerialization {
	fn eq(&self, rhs: &ReplicaSerialization) -> bool {
		// hacky but i don't know a better way
		format!("{:?}", self) == format!("{:?}", rhs)
	}
}

impl<R: Read+ReplicaContext> Deserialize<LE, R> for ReplicaSerialization {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let network_id = LERead::read(reader)?;
		let comp_desers = reader.get_comp_serializations(network_id);
		let mut bit_reader = BEBitReader::new(reader);
		let parent_child_info = ReplicaD::deserialize(&mut bit_reader)?;
		let mut components = vec![];
		for new in comp_desers {
			components.push(new(&mut bit_reader)?);
		}

		Ok(Self {
			network_id,
			parent_child_info,
			components,
		})
	}
}

impl<'a, W: Write> Serialize<LE, W> for &'a ReplicaSerialization {
	fn serialize(self, writer: &mut W) -> Res<()> {
		LEWrite::write(writer, self.network_id)?;
		let mut bit_writer = BEBitWriter::new(vec![]);
		ReplicaS::serialize(&self.parent_child_info, &mut bit_writer)?;

		for comp in &self.components {
			comp.ser(&mut bit_writer)?;
		}
		bit_writer.flush()?;
		LEWrite::write(writer, bit_writer.get_ref())?;
		Ok(())
	}
}

#[cfg(test)]
#[derive(Debug)]
pub(super) struct DummyContext<'a> {
	pub(super) inner: &'a mut &'a[u8],
}

#[cfg(test)]
impl Read for DummyContext<'_> {
	fn read(&mut self, buf: &mut [u8]) -> Res<usize> {
		Read::read(self.inner, buf)
	}
}

#[cfg(test)]
impl ReplicaContext for DummyContext<'_> {
	fn get_comp_constructions<R: Read>(&mut self, _network_id: u16, _lot: Lot, _config: &Option<LuNameValue>) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>> {
		vec![]
	}

	fn get_comp_serializations<R: Read>(&mut self, _network_id: u16) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentSerialization>>> {
		vec![]
	}
}
