use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::ObjId;
use super::{ComponentConstruction, ComponentSerialization};

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct BbbConstruction {
	pub metadata_source_item: Option<ObjId>,
}

impl ComponentConstruction for BbbConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub type BbbSerialization = BbbConstruction;

impl ComponentSerialization for BbbSerialization {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
