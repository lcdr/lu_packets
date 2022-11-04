use std::io::Result as Res;

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::world::{Quaternion, Vector3};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum ClimbingProperty {
    None,
    Ladder,
    ClimbWall,
    ClimbWallStick,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct VelocityInfo {
    pub linear_velocity: Vector3,
    pub angular_velocity: Vector3,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum MotionType {
    Dynamic = 1,
    SphereInertia,
    BoxInertia,
    Keyframed,
    Fixed,
    ThinBoxInertia,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PositionRotationInfo {
    pub position: Vector3,
    pub rotation: Quaternion,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct SimplePhysicsConstruction {
    pub is_climbable: bool,
    pub climbing_property: ClimbingProperty,
    pub velocity_info: Option<VelocityInfo>,
    pub motion_type: Option<MotionType>,
    pub position_rotation_info: Option<PositionRotationInfo>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct SimplePhysicsSerialization {
    pub velocity_info: Option<VelocityInfo>,
    pub motion_type: Option<MotionType>,
    pub position_rotation_info: Option<PositionRotationInfo>,
}

impl ComponentConstruction for SimplePhysicsConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        Serialize::serialize(self, writer)
    }
}

impl ComponentSerialization for SimplePhysicsSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        Serialize::serialize(self, writer)
    }
}

pub struct SimplePhysicsProtocol;

impl ComponentProtocol for SimplePhysicsProtocol {
    type Construction = SimplePhysicsConstruction;
    type Serialization = SimplePhysicsSerialization;
}
