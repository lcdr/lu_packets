use std::io::{Result as Res};

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::ObjId;
use crate::world::{Vector3, Quaternion};
use super::{ComponentConstruction};

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct JetpackInfo {
	pub effect_id: i32, // todo: id
	pub is_flying: bool,
	pub bypass_checks: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StunImmunityInfo {
	// todo: type
	pub immune_to_stun_move: i32,
	pub immune_to_stun_jump: i32,
	pub immune_to_stun_turn: i32,
	pub immune_to_stun_attack: i32,
	pub immune_to_stun_use_item: i32,
	pub immune_to_stun_equip: i32,
	pub immune_to_stun_interact: i32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CheatInfo {
	pub gravity_scale: f32,
	pub run_multiplier: f32,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct Unknown1 {
	pub unknown_1: f32,
	pub unknown_2: bool,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct Unknown2 {
	pub unknown_1: Option<Unknown1>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct FrameStats {
	pub position: Vector3,
	pub rotation: Quaternion,
	pub is_on_ground: bool,
	pub is_on_rail: bool,
	pub linear_velocity: Option<Vector3>,
	pub angular_velocity: Option<Vector3>,
	pub local_space_info: Option<LocalSpaceInfo>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct LocalSpaceInfo {
	pub object_id: ObjId,
	pub position: Vector3,
	pub linear_velocity: Option<Vector3>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ControllablePhysicsConstruction {
	pub jetpack_info: Option<JetpackInfo>,
	pub stun_immunity_info: Option<StunImmunityInfo>,
	pub cheat_info: Option<CheatInfo>,
	pub unknown_1: Option<Unknown1>,
	pub unknown_2: Option<Unknown2>,
	pub frame_stats: Option<FrameStats>,
}

impl ComponentConstruction for ControllablePhysicsConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
