use std::io::Result as Res;

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::ObjId;
use super::ComponentConstruction;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum PossessionType {
	NoPossession,
	AttachedVisible,
	NotAttachedVisible,
	NotAttachedNotVisible,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PossessionInfo {
	pub possessed_id: Option<ObjId>,
	pub possession_type: PossessionType,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct PossessionControlConstruction {
	pub possession_info: Option<PossessionInfo>,
}

impl ComponentConstruction for PossessionControlConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
