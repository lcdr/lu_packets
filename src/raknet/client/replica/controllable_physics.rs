use std::io::{Result as Res};

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::ObjId;
use crate::world::{Vector3, Quaternion};
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};

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
pub struct MagnetAndFlyingUpdate {
	pub loot_pickup_radius: f32,
	pub is_flying: bool,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct BubbleInfo {
	pub bubble_type: i32,
	pub special_animation: bool,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct BubbleUpdateInfo {
	/// If option is not set, the bubble is removed.
	pub bubble_info: Option<BubbleInfo>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct FrameStats {
	pub position: Vector3,
	pub rotation: Quaternion,
	pub is_on_ground: bool,
	pub is_on_rail: bool,

	/// Default cases of the below `Option`s result in 0 for all fields in that options' struct.
	/// This means that if `linear_velocity` is non-zero, you must write the struct, even if nothing
	/// has changed frame over frame.
	pub linear_velocity: Option<Vector3>,
	pub angular_velocity: Option<Vector3>,
	pub local_space_info: Option<LocalSpaceInfo>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct LocalSpaceInfo {
	pub object_id: ObjId,
	pub position: Vector3,

	/// Defaults to [`Vector3::ZERO`] if not set. Always write if non-zero.
	pub linear_velocity: Option<Vector3>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ControllablePhysicsConstruction {
	pub jetpack_info: Option<JetpackInfo>,
	pub stun_immunity_info: Option<StunImmunityInfo>,
	pub cheat_info: Option<CheatInfo>,
	pub magnet_and_flying_update: Option<MagnetAndFlyingUpdate>,
	pub bubble_update_info: Option<BubbleUpdateInfo>,
	pub frame_stats: Option<FrameStats>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct FrameStatsTeleportInfo {
	pub frame_stats: FrameStats,
	pub is_teleporting: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ControllablePhysicsSerialization {
	pub cheat_info: Option<CheatInfo>,
	pub magnet_and_flying_update: Option<MagnetAndFlyingUpdate>,
	pub bubble_update_info: Option<BubbleUpdateInfo>,
	pub frame_stats_teleport_info: Option<FrameStatsTeleportInfo>,
}

impl ComponentConstruction for ControllablePhysicsConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

impl ComponentSerialization for ControllablePhysicsSerialization {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub struct ControllablePhysicsProtocol;

impl ComponentProtocol for ControllablePhysicsProtocol {
	type Construction = ControllablePhysicsConstruction;
	type Serialization = ControllablePhysicsSerialization;
}
