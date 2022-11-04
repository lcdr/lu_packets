use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct CollectibleConstruction {
    pub collectible_id: u16,
}

impl ComponentConstruction for CollectibleConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type CollectibleSerialization = CollectibleConstruction;

impl ComponentSerialization for CollectibleSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct CollectibleProtocol;

impl ComponentProtocol for CollectibleProtocol {
    type Construction = CollectibleConstruction;
    type Serialization = CollectibleSerialization;
}
