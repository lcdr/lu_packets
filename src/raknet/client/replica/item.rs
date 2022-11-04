use std::io::Result as Res;

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::{LuVarWString, ObjId};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum UgcModerationStatus {
    NoStatus,
    Approved,
    Rejected,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ItemInfo {
    pub ug_id: ObjId,
    pub ug_moderation_status: UgcModerationStatus,
    pub ug_description: Option<LuVarWString<u32>>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ItemConstruction {
    pub item_info: Option<ItemInfo>,
}

impl ComponentConstruction for ItemConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type ItemSerialization = ItemConstruction;

impl ComponentSerialization for ItemSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct ItemProtocol;

impl ComponentProtocol for ItemProtocol {
    type Construction = ItemConstruction;
    type Serialization = ItemSerialization;
}
