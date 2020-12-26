use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::ComponentConstruction;

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct LevelProgressionConstruction {
	pub current_level: Option<u32>,
}

impl ComponentConstruction for LevelProgressionConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
