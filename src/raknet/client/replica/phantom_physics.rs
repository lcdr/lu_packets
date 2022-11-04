use std::io::Result as Res;

use endio::{Deserialize, Serialize};
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::simple_physics::PositionRotationInfo;
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::world::Vector3;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum PhysicsEffectType {
    Push,
    Attract,
    Repulse,
    GravityScale,
    Friction,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct DistanceInfo {
    pub min_distance: f32,
    pub max_distance: f32,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct PhysicsEffectInfo {
    pub effect_type: PhysicsEffectType,
    pub amount: f32,
    pub distance_info: Option<DistanceInfo>,
    pub impulse_velocity: Option<Vector3>,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ActivePhysicsEffectInfo {
    pub active_physics_effect: Option<PhysicsEffectInfo>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct PhantomPhysicsConstruction {
    pub position_rotation_info: Option<PositionRotationInfo>,
    pub active_physics_effect_info: Option<ActivePhysicsEffectInfo>,
}

impl ComponentConstruction for PhantomPhysicsConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type PhantomPhysicsSerialization = PhantomPhysicsConstruction;

impl ComponentSerialization for PhantomPhysicsSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct PhantomPhysicsProtocol;

impl ComponentProtocol for PhantomPhysicsProtocol {
    type Construction = PhantomPhysicsConstruction;
    type Serialization = PhantomPhysicsSerialization;
}
