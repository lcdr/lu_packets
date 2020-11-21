pub mod client;
pub mod server;

use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use lu_packets_derive::{GameMessage, GmParam};

use crate::common::{LuVarString, LuVarWString, ObjId, OBJID_EMPTY};
use super::{Lot, LOT_NULL};

type GmString = LuVarString<u32>;
type GmWString = LuVarWString<u32>;

pub(super) trait GmParam: Sized {
	fn deserialize<R: Read>(reader: &mut R) -> Res<Self>;
	fn serialize<W: Write>(&self, writer: &mut W) -> Res<()>;
}

/// Implements `GmParam` by forwarding to [`Deserialize`] and [`Serialize`].
macro_rules! gm_param {
	($typ:ty) => {
		impl crate::world::gm::GmParam for $typ {
			fn deserialize<R: ::std::io::Read>(reader: &mut R) -> ::std::io::Result<Self> {
				::endio::LERead::read(reader)
			}

			fn serialize<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
				::endio::LEWrite::write(writer, self)
			}
		}
	}
}

gm_param!(u8);
gm_param!(u16);
gm_param!(u32);
gm_param!(u64);
gm_param!(i32);
gm_param!(i64);
gm_param!(f32);
gm_param!(GmString);
gm_param!(GmWString);

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

#[derive(Debug, GameMessage, PartialEq)]
pub struct RemoveSkill {
	#[default(false)]
	pub from_skill_set: bool,
	pub skill_id: u32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct EquipInventory {
	#[default(false)]
	pub ignore_cooldown: bool,
	pub out_success: bool,
	pub item_to_equip: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetIgnoreProjectileCollision {
	#[default(false)]
	pub should_ignore: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
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

#[derive(Debug, GameMessage, PartialEq)]
pub struct MoveItemInInventory {
	#[default(INVENTORY_INVALID)]
	pub dest_inv_type: i32, // todo: type
	pub obj_id: ObjId,
	pub inventory_type: InventoryType,
	pub response_code: i32, // todo: type
	pub slot: i32, // todo: unsigned?
}

#[derive(Debug, GameMessage, PartialEq)]
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
