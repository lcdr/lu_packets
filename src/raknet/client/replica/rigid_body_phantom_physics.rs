use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::simple_physics::PositionRotationInfo;
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct RigidBodyPhantomPhysicsConstruction {
    pub position_rotation_info: Option<PositionRotationInfo>,
}

impl ComponentConstruction for RigidBodyPhantomPhysicsConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        Serialize::serialize(self, writer)
    }
}

pub type RigidBodyPhantomPhysicsSerialization = RigidBodyPhantomPhysicsConstruction;

impl ComponentSerialization for RigidBodyPhantomPhysicsSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        Serialize::serialize(self, writer)
    }
}

pub struct RigidBodyPhantomPhysicsProtocol;

impl ComponentProtocol for RigidBodyPhantomPhysicsProtocol {
    type Construction = RigidBodyPhantomPhysicsConstruction;
    type Serialization = RigidBodyPhantomPhysicsSerialization;
}
