use std::io::Result as Res;

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::ObjId;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum AiCombatState {
    Idle,
    Aggro,
    ReturningToTether,
    Spawn,
    Dead,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct CombatAiInfo {
    pub current_combat_state: AiCombatState,
    pub current_target: ObjId,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct BaseCombatAiConstruction {
    pub combat_ai_info: Option<CombatAiInfo>,
}

impl ComponentConstruction for BaseCombatAiConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type BaseCombatAiSerialization = BaseCombatAiConstruction;

impl ComponentSerialization for BaseCombatAiSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct BaseCombatAiProtocol;

impl ComponentProtocol for BaseCombatAiProtocol {
    type Construction = BaseCombatAiConstruction;
    type Serialization = BaseCombatAiSerialization;
}
