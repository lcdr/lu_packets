use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct SwitchConstruction {
    pub is_active: bool,
}

impl ComponentConstruction for SwitchConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type SwitchSerialization = SwitchConstruction;

impl ComponentSerialization for SwitchSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct SwitchProtocol;

impl ComponentProtocol for SwitchProtocol {
    type Construction = SwitchConstruction;
    type Serialization = SwitchSerialization;
}
