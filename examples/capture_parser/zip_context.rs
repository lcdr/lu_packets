use std::collections::HashMap;
use std::io::Result as Res;

use endio_bit::BEBitReader;
use lu_packets::{
	raknet::client::replica::{
		ComponentConstruction, ComponentSerialization, ReplicaContext,
		base_combat_ai::{BaseCombatAiConstruction, BaseCombatAiSerialization},
		bbb::{BbbConstruction, BbbSerialization},
		bouncer::{BouncerConstruction, BouncerSerialization},
		buff::BuffConstruction,
		character::{CharacterConstruction, CharacterSerialization},
		collectible::{CollectibleConstruction, CollectibleSerialization},
		controllable_physics::{ControllablePhysicsConstruction, ControllablePhysicsSerialization},
		destroyable::{DestroyableConstruction, DestroyableSerialization},
		fx::FxConstruction,
		inventory::{InventoryConstruction, InventorySerialization},
		level_progression::{LevelProgressionConstruction, LevelProgressionSerialization},
		phantom_physics::{PhantomPhysicsConstruction, PhantomPhysicsSerialization},
		player_forced_movement::{PlayerForcedMovementConstruction, PlayerForcedMovementSerialization},
		possession_control::{PossessionControlConstruction, PossessionControlSerialization},
		quickbuild::{QuickbuildConstruction, QuickbuildSerialization},
		simple_physics::{SimplePhysicsConstruction, SimplePhysicsSerialization},
		script::ScriptConstruction,
		skill::SkillConstruction,
		vendor::{VendorConstruction, VendorSerialization},
	},
	world::Lot,
};
use zip::read::ZipFile;

use super::Cdclient;

pub struct ZipContext<'a> {
	pub zip: ZipFile<'a>,
	pub lots: &'a mut HashMap<u16, Lot>,
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
	fn get_comp_constructions<R: std::io::Read>(&mut self, network_id: u16, lot: Lot) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>> {
		use endio::Deserialize;

		let comps = self.cdclient.get_comps(lot);

		let mut constrs: Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>> = vec![];
		for comp in comps {
			match comp {
				1 =>   { constrs.push(|x| Ok(Box::new(ControllablePhysicsConstruction::deserialize(x)?))); }
				3 =>   { constrs.push(|x| Ok(Box::new(SimplePhysicsConstruction::deserialize(x)?))); }
				4 =>   { constrs.push(|x| Ok(Box::new(CharacterConstruction::deserialize(x)?))); }
				5 =>   { constrs.push(|x| Ok(Box::new(ScriptConstruction::deserialize(x)?))); }
				6 =>   { constrs.push(|x| Ok(Box::new(BouncerConstruction::deserialize(x)?))); }
				7 =>   { constrs.push(|x| Ok(Box::new(DestroyableConstruction::deserialize(x)?))); }
				9 =>   { constrs.push(|x| Ok(Box::new(SkillConstruction::deserialize(x)?))); }
				16 =>  { constrs.push(|x| Ok(Box::new(VendorConstruction::deserialize(x)?))); }
				17 =>  { constrs.push(|x| Ok(Box::new(InventoryConstruction::deserialize(x)?))); }
				23 =>  { constrs.push(|x| Ok(Box::new(CollectibleConstruction::deserialize(x)?))); }
				40 =>  { constrs.push(|x| Ok(Box::new(PhantomPhysicsConstruction::deserialize(x)?))); }
				44 =>  { constrs.push(|x| Ok(Box::new(FxConstruction::deserialize(x)?))); }
				48 =>  { constrs.push(|x| Ok(Box::new(QuickbuildConstruction::deserialize(x)?))); }
				60 =>  { constrs.push(|x| Ok(Box::new(BaseCombatAiConstruction::deserialize(x)?))); }
				98 =>  { constrs.push(|x| Ok(Box::new(BuffConstruction::deserialize(x)?))); }
				106 => { constrs.push(|x| Ok(Box::new(PlayerForcedMovementConstruction::deserialize(x)?))); }
				107 => { constrs.push(|x| Ok(Box::new(BbbConstruction::deserialize(x)?))); }
				109 => { constrs.push(|x| Ok(Box::new(LevelProgressionConstruction::deserialize(x)?))); }
				110 => { constrs.push(|x| Ok(Box::new(PossessionControlConstruction::deserialize(x)?))); }
				2 | 31 | 35 | 55 | 56 | 64 | 68 | 73 => {},
				x => panic!("{}", x),
			}
		}
		self.lots.insert(network_id, lot);
		constrs
	}

	fn get_comp_serializations<R: std::io::Read>(&mut self, network_id: u16) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentSerialization>>> {
		use endio::Deserialize;

		if let Some(lot) = self.lots.get(&network_id) {
			let comps = self.cdclient.get_comps(*lot);
			let mut sers: Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentSerialization>>> = vec![];
			for comp in comps {
				match comp {
					1 =>   { sers.push(|x| Ok(Box::new(ControllablePhysicsSerialization::deserialize(x)?))); }
					3 =>   { sers.push(|x| Ok(Box::new(SimplePhysicsSerialization::deserialize(x)?))); }
					4 =>   { sers.push(|x| Ok(Box::new(CharacterSerialization::deserialize(x)?))); }
					6 =>   { sers.push(|x| Ok(Box::new(BouncerSerialization::deserialize(x)?))); }
					7 =>   { sers.push(|x| Ok(Box::new(DestroyableSerialization::deserialize(x)?))); }
					16 =>  { sers.push(|x| Ok(Box::new(VendorSerialization::deserialize(x)?))); }
					17 =>  { sers.push(|x| Ok(Box::new(InventorySerialization::deserialize(x)?))); }
					23 =>  { sers.push(|x| Ok(Box::new(CollectibleSerialization::deserialize(x)?))); }
					40 =>  { sers.push(|x| Ok(Box::new(PhantomPhysicsSerialization::deserialize(x)?))); }
					48 =>  { sers.push(|x| Ok(Box::new(QuickbuildSerialization::deserialize(x)?))); }
					60 =>  { sers.push(|x| Ok(Box::new(BaseCombatAiSerialization::deserialize(x)?))); }
					106 => { sers.push(|x| Ok(Box::new(PlayerForcedMovementSerialization::deserialize(x)?))); }
					107 => { sers.push(|x| Ok(Box::new(BbbSerialization::deserialize(x)?))); }
					109 => { sers.push(|x| Ok(Box::new(LevelProgressionSerialization::deserialize(x)?))); }
					110 => { sers.push(|x| Ok(Box::new(PossessionControlSerialization::deserialize(x)?))); }
					2 | 5 | 9 | 31 | 35 | 44 | 55 | 56 | 64 | 68 | 73 | 98 => {},
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
