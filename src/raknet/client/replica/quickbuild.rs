use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::scripted_activity::ActivityUserInfo;
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::LVec;
use crate::world::gm::client::RebuildChallengeState;
use crate::world::Vector3;

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct QuickbuildConstructionInfo {
    pub current_state: RebuildChallengeState,
    pub show_reset_effect: bool,
    pub has_activator: bool,
    pub duration_timer: f32,
    pub total_incomplete_time: f32,
    pub unknown: Option<u32>,
    pub activator_position: Vector3,
    pub reposition_player: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct QuickbuildConstruction {
    pub activity_user_infos: Option<LVec<u32, ActivityUserInfo>>,
    pub quickbuild_construction_info: Option<QuickbuildConstructionInfo>,
}

impl ComponentConstruction for QuickbuildConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct QuickbuildSerializationInfo {
    pub current_state: RebuildChallengeState,
    pub show_reset_effect: bool,
    pub has_activator: bool,
    pub duration_timer: f32,
    pub total_incomplete_time: f32,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct QuickbuildSerialization {
    pub activity_user_infos: Option<LVec<u32, ActivityUserInfo>>,
    pub quickbuild_serialization_info: Option<QuickbuildSerializationInfo>,
}

impl ComponentSerialization for QuickbuildSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct QuickbuildProtocol;

impl ComponentProtocol for QuickbuildProtocol {
    type Construction = QuickbuildConstruction;
    type Serialization = QuickbuildSerialization;
}
