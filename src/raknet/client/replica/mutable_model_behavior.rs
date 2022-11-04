use std::io::Result as Res;

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::ObjId;
use crate::world::{Quaternion, Vector3};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum PhysicsBehaviorType {
    /// todo: option
    Invalid = -1,
    Ground,
    Flying,
    Standard,
    Dynamic,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ModelBehaviorInfo {
    pub is_pickable: bool,
    pub physics_behavior_type: PhysicsBehaviorType,
    pub original_position: Vector3,
    pub original_rotation: Quaternion,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ModelEditingInfo {
    pub old_object_id: ObjId,
    pub player_editing_model: ObjId,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct MutableModelBehaviorConstructionInfo {
    pub behavior_count: u32,
    pub is_paused: bool,
    pub model_editing_info: Option<ModelEditingInfo>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct MutableModelBehaviorConstruction {
    pub model_behavior_info: Option<ModelBehaviorInfo>,
    pub mutable_model_behavior_construction_info: Option<MutableModelBehaviorConstructionInfo>,
}

impl ComponentConstruction for MutableModelBehaviorConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct MutableModelBehaviorSerializationInfo {
    pub behavior_count: u32,
    pub is_paused: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct MutableModelBehaviorSerialization {
    pub model_behavior_info: Option<ModelBehaviorInfo>,
    pub mutable_model_behavior_serialization_info: Option<MutableModelBehaviorSerializationInfo>,
}

impl ComponentSerialization for MutableModelBehaviorSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct MutableModelBehaviorProtocol;

impl ComponentProtocol for MutableModelBehaviorProtocol {
    type Construction = MutableModelBehaviorConstruction;
    type Serialization = MutableModelBehaviorSerialization;
}
