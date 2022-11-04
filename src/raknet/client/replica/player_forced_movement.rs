use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ForcedMovementInfo {
    pub player_on_rail: bool,
    pub show_billboard: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct PlayerForcedMovementConstruction {
    pub forced_movement_info: Option<ForcedMovementInfo>,
}

impl ComponentConstruction for PlayerForcedMovementConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type PlayerForcedMovementSerialization = PlayerForcedMovementConstruction;

impl ComponentSerialization for PlayerForcedMovementSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct PlayerForcedMovementProtocol;

impl ComponentProtocol for PlayerForcedMovementProtocol {
    type Construction = PlayerForcedMovementConstruction;
    type Serialization = PlayerForcedMovementSerialization;
}
