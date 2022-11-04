use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::ObjId;

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

pub struct BbbProtocol;

impl ComponentProtocol for BbbProtocol {
    type Construction = BbbConstruction;
    type Serialization = BbbSerialization;
}
