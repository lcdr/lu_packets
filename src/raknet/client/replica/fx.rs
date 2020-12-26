use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::{LuVarString, LuVarWString, LVec, ObjId};
use super::ComponentConstruction;

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct EffectInfo {
	pub effect_name: LuVarString<u8>,
	pub effect_id: u32, // todo: type
	pub effect_type: LuVarWString<u8>,
	pub priority: f32,
	pub secondary: ObjId,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct FxConstruction {
	pub active_effects: LVec<u32, EffectInfo>,
}

impl ComponentConstruction for FxConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

