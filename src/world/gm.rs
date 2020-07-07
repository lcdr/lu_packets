use lu_packets_derive::GameMessage;

use crate::common::{ObjId, OBJID_EMPTY};
use super::{InventoryType, Lot, LOT_NULL};

#[derive(Debug, GameMessage)]
pub struct EquipInventory {
	#[default(false)]
	pub ignore_cooldown: bool,
	pub out_success: bool,
	pub item_to_equip: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct SetIgnoreProjectileCollision {
	#[default(false)]
	pub should_ignore: bool,
}

#[derive(Debug, GameMessage)]
pub struct UnEquipInventory {
	#[default(false)]
	pub even_if_dead: bool,
	#[default(false)]
	pub ignore_cooldown: bool,
	pub out_success: bool,
	pub item_to_unequip: ObjId,
	#[default(OBJID_EMPTY)]
	pub replacement_object_id: ObjId,
}

const INVENTORY_INVALID: i32 = -1;

#[derive(Debug, GameMessage)]
pub struct MoveItemInInventory {
	#[default(INVENTORY_INVALID)]
	pub dest_inv_type: i32, // todo: type
	pub obj_id: ObjId,
	pub inventory_type: InventoryType,
	pub response_code: i32, // todo: type
	pub slot: i32, // todo: unsigned?
}

#[derive(Debug, GameMessage)]
pub struct MoveInventoryBatch {
	#[default(false)]
	pub allow_partial: bool,
	#[default(false)]
	pub out_success: bool,
	#[default(1)]
	pub count: u32,
	#[default(InventoryType::Default)]
	pub dst_bag: InventoryType,
	#[default(LOT_NULL)]
	pub move_lot: Lot,
	#[default(OBJID_EMPTY)]
	pub move_subkey: ObjId,
	#[default(false)]
	pub show_flying_loot: bool,
	#[default(InventoryType::Default)]
	pub src_bag: InventoryType,
	#[default(OBJID_EMPTY)]
	pub start_object_id: ObjId,
}
