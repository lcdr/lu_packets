use std::io::Result as Res;

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::{LuVarWString, ObjId};
use crate::world::gm::client::{PetAbilityType, PetModerationStatus};
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum PossessionType {
	NoPossession,
	AttachedVisible,
	NotAttachedVisible,
	NotAttachedNotVisible,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct TamedPetInfo {
	pub pet_name_moderation_status: PetModerationStatus,
	pub pet_name: LuVarWString<u8>,
	pub owner_name: LuVarWString<u8>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PetConstructionInfo {
	/// todo: bitflag
	pub pet_state: u32,
	pub ability_in_use: PetAbilityType,
	pub interaction_id: Option<ObjId>,
	pub owner_id: Option<ObjId>,
	pub tamed_pet_info: Option<TamedPetInfo>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct PetConstruction {
	pub pet_construction_info: Option<PetConstructionInfo>,
}

impl ComponentConstruction for PetConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub type PetSerialization = PetConstruction;

impl ComponentSerialization for PetSerialization {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub struct PetProtocol;

impl ComponentProtocol for PetProtocol {
	type Construction = PetConstruction;
	type Serialization = PetSerialization;
}
