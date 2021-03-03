use std::collections::HashMap;
use std::io::Result as Res;

use endio_bit::BEBitReader;
use lu_packets::{
	lu,
	raknet::client::replica::{
		ComponentConstruction, ComponentSerialization, ReplicaContext,
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
		moving_platform::{MovingPlatformConstruction, MovingPlatformSerialization},
		mutable_model_behavior::{MutableModelBehaviorConstruction, MutableModelBehaviorSerialization},
		phantom_physics::{PhantomPhysicsConstruction, PhantomPhysicsSerialization},
		player_forced_movement::{PlayerForcedMovementConstruction, PlayerForcedMovementSerialization},
		pet::{PetConstruction, PetSerialization},
		possessable::{PossessableConstruction, PossessableSerialization},
		possession_control::{PossessionControlConstruction, PossessionControlSerialization},
		quickbuild::{QuickbuildConstruction, QuickbuildSerialization},
		simple_physics::{SimplePhysicsConstruction, SimplePhysicsSerialization},
		script::ScriptConstruction,
		scripted_activity::{ScriptedActivityConstruction, ScriptedActivitySerialization},
		skill::SkillConstruction,
		switch::{SwitchConstruction, SwitchSerialization},
		vendor::{VendorConstruction, VendorSerialization},
	},
	world::{Lot, LuNameValue, LnvValue},
};
use zip::read::ZipFile;

use super::Cdclient;

const COMP_ORDER : [u32; 28] = [108, 1, 3, 40, 98, 7, 23, 110, 109, 106, 4, 26, 17, 5, 9, 60, 11, 48, 25, 16, 100, 39, 42, 6, 49, 2, 44, 107];

pub struct ZipContext<'a> {
	pub zip: ZipFile<'a>,
	pub comps: &'a mut HashMap<u16, Vec<u32>>,
	pub cdclient: &'a mut Cdclient,
	pub assert_fully_read: bool,
}

impl std::io::Read for ZipContext<'_> {
	fn read(&mut self, buf: &mut [u8]) -> Res<usize> {
		self.zip.read(buf)
	}
}

// hacky hardcoded components to be able to read player replicas without DB lookup
impl ReplicaContext for ZipContext<'_> {
	fn get_comp_constructions<R: std::io::Read>(&mut self, network_id: u16, lot: Lot, config: &Option<LuNameValue>) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>> {
		use endio::Deserialize;

		let mut rows = self.cdclient.get_comps(lot).clone();

		if rows.contains(&42) {
			if let Some(conf) = config {
				if let Some(model_type) = conf.get(&lu!("modelType")) {
					dbg!(model_type);
					if let LnvValue::I32(m_type) = model_type {
						if *m_type == 0 {
							rows.push(1);
						} else {
							rows.push(3);
						}
					}
				}
			}
		}

		rows.sort_by_key(|x| COMP_ORDER.iter().position(|y| y == x).unwrap_or(usize::MAX));

		let mut comps = vec![];
			for row in rows {
				// special case: utter bodge
				match row {
					2  => { comps.push(44); }
					4  => { comps.push(110); comps.push(109); comps.push(106); }
					7  => { comps.push(98); }
					23 | 48 => {
						if !comps.contains(&7) {
							comps.push(7);
						}
					}
					_ => {},
				}
				comps.push(row);
			}
			// special case: utter bodge
			if comps.contains(&26) {
				comps.remove(comps.iter().position(|&x| x == 11).unwrap());
				comps.remove(comps.iter().position(|&x| x == 42).unwrap());
			}
			dbg!(&comps);

		let mut constrs: Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>> = vec![];
		for comp in &comps {
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
				98 =>  { constrs.push(|x| Ok(Box::new(BuffConstruction::deserialize(x)?))); }
				100 => { constrs.push(|x| Ok(Box::new(DonationVendorConstruction::deserialize(x)?))); }
				106 => { constrs.push(|x| Ok(Box::new(PlayerForcedMovementConstruction::deserialize(x)?))); }
				107 => { constrs.push(|x| Ok(Box::new(BbbConstruction::deserialize(x)?))); }
				108 => { constrs.push(|x| Ok(Box::new(PossessableConstruction::deserialize(x)?))); }
				109 => { constrs.push(|x| Ok(Box::new(LevelProgressionConstruction::deserialize(x)?))); }
				110 => { constrs.push(|x| Ok(Box::new(PossessionControlConstruction::deserialize(x)?))); }
				2 | 12 | 24 | 27 | 31 | 35 | 36 | 43 | 45 | 55 | 56 | 57 | 64 | 65 | 67 | 68 | 73 | 78 | 95 | 104 | 113 | 114 => {},
				x => panic!("{}", x),
			}
		}
		self.comps.insert(network_id, comps);
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
					106 => { sers.push(|x| Ok(Box::new(PlayerForcedMovementSerialization::deserialize(x)?))); }
					107 => { sers.push(|x| Ok(Box::new(BbbSerialization::deserialize(x)?))); }
					108 => { sers.push(|x| Ok(Box::new(PossessableSerialization::deserialize(x)?))); }
					109 => { sers.push(|x| Ok(Box::new(LevelProgressionSerialization::deserialize(x)?))); }
					110 => { sers.push(|x| Ok(Box::new(PossessionControlSerialization::deserialize(x)?))); }
					2 | 5 | 9 | 12 | 24 | 27 | 31 | 35 | 36 | 43 | 44 | 45 | 55 | 56 | 57 | 64 | 65 | 67 | 68 | 73 | 78 | 95 | 98 | 104 | 113 | 114 => {},
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
