use std::io::Result as Res;

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::controllable_physics::LocalSpaceInfo;
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::world::{Quaternion, Vector3};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum EndOfRaceBehaviorType {
    DriveStraight,
    StopStraight,
    SlideLeft,
    SlideRight,
    Do360Left,
    Do360Right,
    TwoWheels,
    Jump,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct RemoteInputInfo {
    pub remote_input_x: f32,
    pub remote_input_y: f32,
    pub do_powerslide: bool,
    pub is_modified: bool,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct VehicleFrameStats {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub is_on_ground: bool,
    pub is_on_rail: bool,
    pub linear_velocity: Option<Vector3>,
    pub angular_velocity: Option<Vector3>,
    pub local_space_info: Option<LocalSpaceInfo>,
    pub remote_input_info: Option<RemoteInputInfo>,
    pub remote_input_ping: f32,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct VehiclePhysicsConstruction {
    pub vehicle_frame_stats: Option<VehicleFrameStats>,
    pub end_of_race_behavior_type: EndOfRaceBehaviorType,
    pub is_input_locked: bool,
    pub wheel_lock_extra_friction: Option<bool>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct VehicleFrameStatsTeleportInfo {
    pub vehicle_frame_stats: VehicleFrameStats,
    pub is_teleporting: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct VehiclePhysicsSerialization {
    pub vehicle_frame_stats_teleport_info: Option<VehicleFrameStatsTeleportInfo>,
    pub wheel_lock_extra_friction: Option<bool>,
}

impl ComponentConstruction for VehiclePhysicsConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

impl ComponentSerialization for VehiclePhysicsSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct VehiclePhysicsProtocol;

impl ComponentProtocol for VehiclePhysicsProtocol {
    type Construction = VehiclePhysicsConstruction;
    type Serialization = VehiclePhysicsSerialization;
}
