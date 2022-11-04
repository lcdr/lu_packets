use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::{LVec, ObjId};

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ActivityUserInfo {
    pub user_object_id: ObjId,
    // todo[min_const_generics]
    // pub activity_values: [f32; 10],
    pub activity_value_0: f32,
    pub activity_value_1: f32,
    pub activity_value_2: f32,
    pub activity_value_3: f32,
    pub activity_value_4: f32,
    pub activity_value_5: f32,
    pub activity_value_6: f32,
    pub activity_value_7: f32,
    pub activity_value_8: f32,
    pub activity_value_9: f32,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ScriptedActivityConstruction {
    pub activity_user_infos: Option<LVec<u32, ActivityUserInfo>>,
}

impl ComponentConstruction for ScriptedActivityConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type ScriptedActivitySerialization = ScriptedActivityConstruction;

impl ComponentSerialization for ScriptedActivitySerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct ScriptedActivityProtocol;

impl ComponentProtocol for ScriptedActivityProtocol {
    type Construction = ScriptedActivityConstruction;
    type Serialization = ScriptedActivitySerialization;
}
