use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::world::Lot;

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct LupExhibitConstruction {
    pub exhibited_lot: Option<Lot>,
}

impl ComponentConstruction for LupExhibitConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type LupExhibitSerialization = LupExhibitConstruction;

impl ComponentSerialization for LupExhibitSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct LupExhibitProtocol;

impl ComponentProtocol for LupExhibitProtocol {
    type Construction = LupExhibitConstruction;
    type Serialization = LupExhibitSerialization;
}
