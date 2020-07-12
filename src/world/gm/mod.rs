pub mod client;
pub mod server;

use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use lu_packets_derive::{GameMessage, GmParam};

use crate::common::{LuVarStr, LuVarWStr, ObjId, OBJID_EMPTY};
use super::{Lot, LOT_NULL};

type GmString = LuVarStr<u32>;
type GmWString = LuVarWStr<u32>;

pub(super) trait GmParam: Sized {
	fn deserialize<R: Read>(reader: &mut R) -> Res<Self>;
	fn serialize<W: Write>(&self, writer: &mut W) -> Res<()>;
}

macro_rules! impl_gm {
	($typ:ty) => {
		impl GmParam for $typ {
			fn deserialize<R: Read>(reader: &mut R) -> Res<Self> {
				LERead::read(reader)
			}

			fn serialize<W: Write>(&self, writer: &mut W) -> Res<()> {
				LEWrite::write(writer, self)
			}
		}
	}
}

impl_gm!(u8);
impl_gm!(u16);
impl_gm!(u32);
impl_gm!(u64);
impl_gm!(i32);
impl_gm!(i64);
impl_gm!(f32);
impl_gm!(GmString);
impl_gm!(GmWString);

impl GmParam for Vec<u8> {
	fn deserialize<R: Read>(reader: &mut R) -> Res<Self> {
		let str_len: u32 = LERead::read(reader)?;
		let str_len = str_len as usize;
		let mut vec = Vec::with_capacity(str_len);
		unsafe { vec.set_len(str_len); }
		Read::read(reader, &mut vec)?;
		Ok(vec)
	}

	fn serialize<W: Write>(&self, writer: &mut W) -> Res<()> {
		LEWrite::write(writer, self.len() as u32)?;
		Write::write_all(writer, self)
	}
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum InventoryType {
	Default,
	Bank,
	Brick,
	ModelsInBbb,
	TempEquip,
	Model,
	ModuleInUse,
	Behavior,
	Property,
	BrickInBbb,
	Vendor,
	Buyback,
	Quest,
	Donation,
	BankModel,
	BankBehavior,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum KillType {
	Violent,
	Silent,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum MissionState {
	Unavailable = 0,
	Available = 1,
	Active = 2,
	ReadyToComplete = 4,
	Completed = 8,
	CompleteAndAvailable = 9,
	CompleteAndActive = 10,
	CompleteAndReadyToComplete = 12,
	Fail = 16,
	ReadyToCompleteReported = 32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum PetNotificationType {
	OwnerDied = 1,
	OwnerOnPetBouncer,
	OwnerUsedBouncer,
	PetOnJumpActivatedObj,
	PetOffSwitch,
	PetAtDigLocation,
	PetLeftDigLocation,
	EndSignal,
	PetToDespawn,
	GoToObject,
	OwnerResurrected,
	OwnerOnDig,
	Released,
	OwnerOffPetBouncer,
	OwnerOffDig,
}

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
