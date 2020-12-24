use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::ComponentCreation;

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ForcedMovementInfo {
	pub player_on_rail: bool,
	pub show_billboard: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct PlayerForcedMovementCreation {
	pub forced_movement_info: Option<ForcedMovementInfo>,
}

impl ComponentCreation for PlayerForcedMovementCreation {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
