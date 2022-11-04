use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::{LVec, ObjId};

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct BehaviorInfo {
    pub unknown_1: u32,
    pub action: u32, // todo: type
    pub wait_time_ms: u32,
    pub template_id: u32, // todo: type
    pub caster: ObjId,
    pub originator: ObjId,
    pub target: ObjId,
    pub used_mouse: bool,
    pub cooldown: f32,
    pub charge_time: f32,
    pub imagination_cost: u32,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct SkillInfo {
    pub unknown_1: u32,
    pub skill_id: u32,    // todo: type
    pub cast_type: u32,   // todo: type
    pub cancel_type: u32, // todo: type
    pub behaviors: LVec<u32, BehaviorInfo>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct SkillConstruction {
    pub skills_in_progress: Option<LVec<u32, SkillInfo>>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct SkillSerialization {}

impl ComponentConstruction for SkillConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

impl ComponentSerialization for SkillSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct SkillProtocol;

impl ComponentProtocol for SkillProtocol {
    type Construction = SkillConstruction;
    type Serialization = SkillSerialization;
}
