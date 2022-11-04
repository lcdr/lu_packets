use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::{LVec, ObjId};
use crate::world::gm::InventoryType;
use crate::world::{Lot, LuNameValue, Quaternion, Vector3};

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct EquippedItemInfo {
    pub id: ObjId,
    pub lot: Lot,
    pub subkey: Option<ObjId>,
    pub count: Option<u32>,
    pub slot: Option<u16>,
    pub inventory_type: Option<InventoryType>,
    pub extra_info: Option<LuNameValue>,
    pub is_bound: bool,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct EquippedModelTransform {
    pub model_id: ObjId,
    pub equip_position: Vector3,
    pub equip_rotation: Quaternion,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct InventoryConstruction {
    pub equipped_items: Option<LVec<u32, EquippedItemInfo>>,
    pub equipped_model_transforms: Option<LVec<u32, EquippedModelTransform>>,
}

impl ComponentConstruction for InventoryConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type InventorySerialization = InventoryConstruction;

impl ComponentSerialization for InventorySerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct InventoryProtocol;

impl ComponentProtocol for InventoryProtocol {
    type Construction = InventoryConstruction;
    type Serialization = InventorySerialization;
}
