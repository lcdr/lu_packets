use std::collections::HashMap;
use std::io::Result as Res;

use endio_bit::BEBitReader;
use lu_packets::{
	lu,
	raknet::client::replica::{
		ComponentConstruction, ComponentSerialization, ReplicaContext,
		achievement_vendor::{AchievementVendorConstruction, AchievementVendorSerialization},
		base_combat_ai::{BaseCombatAiConstruction, BaseCombatAiSerialization},
		bbb::{BbbConstruction, BbbSerialization},
		bouncer::{BouncerConstruction, BouncerSerialization},
		buff::BuffConstruction,
		character::{CharacterConstruction, CharacterSerialization},
		collectible::{CollectibleConstruction, CollectibleSerialization},
		controllable_physics::{ControllablePhysicsConstruction, ControllablePhysicsSerialization},
		donation_vendor::{DonationVendorConstruction, DonationVendorSerialization},
		destroyable::{DestroyableConstruction, DestroyableSerialization},
		fx::FxConstruction,
		inventory::{InventoryConstruction, InventorySerialization},
		item::{ItemConstruction, ItemSerialization},
		level_progression::{LevelProgressionConstruction, LevelProgressionSerialization},
		module_assembly::ModuleAssemblyConstruction,
		moving_platform::{MovingPlatformConstruction, MovingPlatformSerialization},
		mutable_model_behavior::{MutableModelBehaviorConstruction, MutableModelBehaviorSerialization},
		phantom_physics::{PhantomPhysicsConstruction, PhantomPhysicsSerialization},
		player_forced_movement::{PlayerForcedMovementConstruction, PlayerForcedMovementSerialization},
		pet::{PetConstruction, PetSerialization},
		possessable::{PossessableConstruction, PossessableSerialization},
		possession_control::{PossessionControlConstruction, PossessionControlSerialization},
		quickbuild::{QuickbuildConstruction, QuickbuildSerialization},
		rigid_body_phantom_physics::{RigidBodyPhantomPhysicsConstruction, RigidBodyPhantomPhysicsSerialization},
		script::ScriptConstruction,
		scripted_activity::{ScriptedActivityConstruction, ScriptedActivitySerialization},
		shooting_gallery::{ShootingGalleryConstruction, ShootingGallerySerialization},
		simple_physics::{SimplePhysicsConstruction, SimplePhysicsSerialization},
		skill::SkillConstruction,
		switch::{SwitchConstruction, SwitchSerialization},
		vendor::{VendorConstruction, VendorSerialization},
	},
	world::{Lot, LuNameValue, LnvValue},
};
use zip::read::ZipFile;

use super::Cdclient;

const COMP_ORDER: [u32; 33] = [108, 61, 1, 30, 20, 3, 40, 98, 7, 23, 110, 109, 106, 4, 26, 17, 5, 9, 60, 11, 48, 25, 16, 100, 102, 19, 39, 42, 6, 49, 2, 44, 107];

pub struct ZipContext<'a> {
	pub zip: ZipFile<'a>,
	pub comps: &'a mut HashMap<u16, Vec<u32>>,
	pub cdclient: &'a mut Cdclient,
	pub assert_fully_read: bool,
}

impl ZipContext<'_> {
	fn apply_whitelist(comps: &mut Vec<u32>, config: &Option<LuNameValue>) {
		if let Some(conf) = config {
			if let Some(LnvValue::I32(1)) = conf.get(&lu!("componentWhitelist")) {
				dbg!("applying whitelist");
				comps.retain(|&x|
					match x  {
						1 | 2 | 3 | 7 | 10 | 11 | 24 | 42 => true,
						_ => false,
					}
				);
			}
		}
	}

	fn apply_config_overrides(comps: &mut Vec<u32>, config: &Option<LuNameValue>) {
		if comps.contains(&42) {
			if let Some(conf) = config {
				if conf.contains_key(&lu!("modelBehaviors")) {
					if let Some(LnvValue::I32(m_type)) = conf.get(&lu!("modelType")) {
						let new_phys = if *m_type == 0 { 1 } else { 3 };
						if let Some(phys_index) = comps.iter().position(|&x| x == 1 || x == 3) {
							comps[phys_index] = new_phys;
						} else {
							comps.push(new_phys);
						}
						return;
					}
				}
			}
			if comps.iter().position(|&x| x == 1 || x == 3).is_none() {
				comps.push(3);
			}
		}
	}

	fn apply_component_overrides(comps: &Vec<u32>, final_comps: &mut Vec<u32>) {
		for comp in comps {
			// special case: utter bodge
			match comp {
				2  => { final_comps.push(44); }
				4  => { final_comps.push(110); final_comps.push(109); final_comps.push(106); }
				7  => { final_comps.push(98); }
				23 | 48 => {
					if !final_comps.contains(&7) {
						final_comps.push(7);
					}
				}
				_ => {},
			}
			final_comps.push(*comp);
		}
		// special case: utter bodge
		if final_comps.contains(&26) {
			final_comps.remove(final_comps.iter().position(|&x| x == 11).unwrap());
			final_comps.remove(final_comps.iter().position(|&x| x == 42).unwrap());
		}
	}

	fn map_constrs<R: std::io::Read>(comps: &Vec<u32>) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>> {
		use endio::Deserialize;

		let mut constrs: Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>> = vec![];
		for comp in comps {
			match comp {
				1  =>  { constrs.push(|x| Ok(Box::new(ControllablePhysicsConstruction::deserialize(x)?))); }
				3  =>  { constrs.push(|x| Ok(Box::new(SimplePhysicsConstruction::deserialize(x)?))); }
				4  =>  { constrs.push(|x| Ok(Box::new(CharacterConstruction::deserialize(x)?))); }
				5  =>  { constrs.push(|x| Ok(Box::new(ScriptConstruction::deserialize(x)?))); }
				6  =>  { constrs.push(|x| Ok(Box::new(BouncerConstruction::deserialize(x)?))); }
				7  =>  { constrs.push(|x| Ok(Box::new(DestroyableConstruction::deserialize(x)?))); }
				9  =>  { constrs.push(|x| Ok(Box::new(SkillConstruction::deserialize(x)?))); }
				11 =>  { constrs.push(|x| Ok(Box::new(ItemConstruction::deserialize(x)?))); }
				16 =>  { constrs.push(|x| Ok(Box::new(VendorConstruction::deserialize(x)?))); }
				17 =>  { constrs.push(|x| Ok(Box::new(InventoryConstruction::deserialize(x)?))); }
				19 =>  { constrs.push(|x| Ok(Box::new(ShootingGalleryConstruction::deserialize(x)?))); }
				20 =>  { constrs.push(|x| Ok(Box::new(RigidBodyPhantomPhysicsConstruction::deserialize(x)?))); }
				23 =>  { constrs.push(|x| Ok(Box::new(CollectibleConstruction::deserialize(x)?))); }
				25 =>  { constrs.push(|x| Ok(Box::new(MovingPlatformConstruction::deserialize(x)?))); }
				26 =>  { constrs.push(|x| Ok(Box::new(PetConstruction::deserialize(x)?))); }
				39 =>  { constrs.push(|x| Ok(Box::new(ScriptedActivityConstruction::deserialize(x)?))); }
				40 =>  { constrs.push(|x| Ok(Box::new(PhantomPhysicsConstruction::deserialize(x)?))); }
				42 =>  { constrs.push(|x| Ok(Box::new(MutableModelBehaviorConstruction::deserialize(x)?))); }
				44 =>  { constrs.push(|x| Ok(Box::new(FxConstruction::deserialize(x)?))); }
				48 =>  { constrs.push(|x| Ok(Box::new(QuickbuildConstruction::deserialize(x)?))); }
				49 =>  { constrs.push(|x| Ok(Box::new(SwitchConstruction::deserialize(x)?))); }
				60 =>  { constrs.push(|x| Ok(Box::new(BaseCombatAiConstruction::deserialize(x)?))); }
				61 =>  { constrs.push(|x| Ok(Box::new(ModuleAssemblyConstruction::deserialize(x)?))); }
				98 =>  { constrs.push(|x| Ok(Box::new(BuffConstruction::deserialize(x)?))); }
				100 => { constrs.push(|x| Ok(Box::new(DonationVendorConstruction::deserialize(x)?))); }
				102 => { constrs.push(|x| Ok(Box::new(AchievementVendorConstruction::deserialize(x)?))); }
				106 => { constrs.push(|x| Ok(Box::new(PlayerForcedMovementConstruction::deserialize(x)?))); }
				107 => { constrs.push(|x| Ok(Box::new(BbbConstruction::deserialize(x)?))); }
				108 => { constrs.push(|x| Ok(Box::new(PossessableConstruction::deserialize(x)?))); }
				109 => { constrs.push(|x| Ok(Box::new(LevelProgressionConstruction::deserialize(x)?))); }
				110 => { constrs.push(|x| Ok(Box::new(PossessionControlConstruction::deserialize(x)?))); }
				2 | 12 | 24 | 27 | 31 | 35 | 36 | 43 | 45 | 55 | 56 | 57 | 64 | 65 | 67 | 68 | 73 | 74 | 78 | 95 | 104 | 113 | 114 => {},
				x => panic!("{}", x),
			}
		}
		constrs
	}
}

impl std::io::Read for ZipContext<'_> {
	fn read(&mut self, buf: &mut [u8]) -> Res<usize> {
		self.zip.read(buf)
	}
}

// hacky hardcoded components to be able to read player replicas without DB lookup
impl ReplicaContext for ZipContext<'_> {
	fn get_comp_constructions<R: std::io::Read>(&mut self, network_id: u16, lot: Lot, config: &Option<LuNameValue>) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>> {
		let mut comps = self.cdclient.get_comps(lot).clone();

		Self::apply_whitelist(&mut comps, config);
		Self::apply_config_overrides(&mut comps, config);

		comps.sort_by_key(|x| COMP_ORDER.iter().position(|y| y == x).unwrap_or(usize::MAX));
		comps.dedup();

		let mut final_comps = vec![];
		Self::apply_component_overrides(&comps, &mut final_comps);
		dbg!(&final_comps);
		let constrs = Self::map_constrs(&final_comps);
		self.comps.insert(network_id, final_comps);
		constrs
	}

	fn get_comp_serializations<R: std::io::Read>(&mut self, network_id: u16) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentSerialization>>> {
		use endio::Deserialize;

		if let Some(comps) = self.comps.get(&network_id) {
			let mut sers: Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentSerialization>>> = vec![];
			for comp in comps {
				match comp {
					1   => { sers.push(|x| Ok(Box::new(ControllablePhysicsSerialization::deserialize(x)?))); }
					3   => { sers.push(|x| Ok(Box::new(SimplePhysicsSerialization::deserialize(x)?))); }
					4   => { sers.push(|x| Ok(Box::new(CharacterSerialization::deserialize(x)?))); }
					6   => { sers.push(|x| Ok(Box::new(BouncerSerialization::deserialize(x)?))); }
					7   => { sers.push(|x| Ok(Box::new(DestroyableSerialization::deserialize(x)?))); }
					11  => { sers.push(|x| Ok(Box::new(ItemSerialization::deserialize(x)?))); }
					16  => { sers.push(|x| Ok(Box::new(VendorSerialization::deserialize(x)?))); }
					17  => { sers.push(|x| Ok(Box::new(InventorySerialization::deserialize(x)?))); }
					19  => { sers.push(|x| Ok(Box::new(ShootingGallerySerialization::deserialize(x)?))); }
					20  => { sers.push(|x| Ok(Box::new(RigidBodyPhantomPhysicsSerialization::deserialize(x)?))); }
					23  => { sers.push(|x| Ok(Box::new(CollectibleSerialization::deserialize(x)?))); }
					25  => { sers.push(|x| Ok(Box::new(MovingPlatformSerialization::deserialize(x)?))); }
					26  => { sers.push(|x| Ok(Box::new(PetSerialization::deserialize(x)?))); }
					39  => { sers.push(|x| Ok(Box::new(ScriptedActivitySerialization::deserialize(x)?))); }
					40  => { sers.push(|x| Ok(Box::new(PhantomPhysicsSerialization::deserialize(x)?))); }
					42  => { sers.push(|x| Ok(Box::new(MutableModelBehaviorSerialization::deserialize(x)?))); }
					48  => { sers.push(|x| Ok(Box::new(QuickbuildSerialization::deserialize(x)?))); }
					49  => { sers.push(|x| Ok(Box::new(SwitchSerialization::deserialize(x)?))); }
					60  => { sers.push(|x| Ok(Box::new(BaseCombatAiSerialization::deserialize(x)?))); }
					100 => { sers.push(|x| Ok(Box::new(DonationVendorSerialization::deserialize(x)?))); }
					102 => { sers.push(|x| Ok(Box::new(AchievementVendorSerialization::deserialize(x)?))); }
					106 => { sers.push(|x| Ok(Box::new(PlayerForcedMovementSerialization::deserialize(x)?))); }
					107 => { sers.push(|x| Ok(Box::new(BbbSerialization::deserialize(x)?))); }
					108 => { sers.push(|x| Ok(Box::new(PossessableSerialization::deserialize(x)?))); }
					109 => { sers.push(|x| Ok(Box::new(LevelProgressionSerialization::deserialize(x)?))); }
					110 => { sers.push(|x| Ok(Box::new(PossessionControlSerialization::deserialize(x)?))); }
					2 | 5 | 9 | 12 | 24 | 27 | 31 | 35 | 36 | 43 | 44 | 45 | 55 | 56 | 57 | 61 | 64 | 65 | 67 | 68 | 73 | 74 | 78 | 95 | 98 | 104 | 113 | 114 => {},
					x => panic!("{}", x),
				}
			}
			self.assert_fully_read = true;
			return sers;
		}
		self.assert_fully_read = false;
		vec![]
	}
}
