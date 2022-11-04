use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::world::LuNameValue;

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ScriptConstruction {
    pub network_vars: Option<LuNameValue>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ScriptSerialization {}

impl ComponentConstruction for ScriptConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

impl ComponentSerialization for ScriptSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct ScriptProtocol;

impl ComponentProtocol for ScriptProtocol {
    type Construction = ScriptConstruction;
    type Serialization = ScriptSerialization;
}
