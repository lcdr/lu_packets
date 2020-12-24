use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::ObjId;
use super::ComponentCreation;

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct BbbCreation {
	pub metadata_source_item: Option<ObjId>,
}

impl ComponentCreation for BbbCreation {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
