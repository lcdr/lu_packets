use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct BouncerConstruction {
    pub bounce_on_collision: Option<bool>,
}

impl ComponentConstruction for BouncerConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type BouncerSerialization = BouncerConstruction;

impl ComponentSerialization for BouncerSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct BouncerProtocol;

impl ComponentProtocol for BouncerProtocol {
    type Construction = BouncerConstruction;
    type Serialization = BouncerSerialization;
}
