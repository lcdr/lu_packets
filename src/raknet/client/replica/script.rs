use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::world::LuNameValue;
use super::ComponentConstruction;

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ScriptConstruction {
	pub network_vars: Option<LuNameValue>,
}

impl ComponentConstruction for ScriptConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
