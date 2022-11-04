use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::ObjId;

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PossessableInfo {
    pub possessor_id: Option<ObjId>,
    pub animation_flag: Option<u32>,
    pub immediate_depossess: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct PossessableConstruction {
    pub possessable_info: Option<PossessableInfo>,
}

impl ComponentConstruction for PossessableConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type PossessableSerialization = PossessableConstruction;

impl ComponentSerialization for PossessableSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct PossessableProtocol;

impl ComponentProtocol for PossessableProtocol {
    type Construction = PossessableConstruction;
    type Serialization = PossessableSerialization;
}
