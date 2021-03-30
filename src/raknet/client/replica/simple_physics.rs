use std::io::{Read, Result as Res, Write};

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};
use endio_bit::{BEBitReader, BEBitWriter};
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::world::{Vector3, Quaternion};
use super::{ReplicaD, ReplicaS, ComponentConstruction, ComponentProtocol, ComponentSerialization};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum ClimbingProperty {
	Ladder = 1,
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

#[derive(BitVariantTests, Debug, PartialEq)]
pub struct SimplePhysicsConstruction {
	pub climbing_property: Option<ClimbingProperty>,
	pub velocity_info: Option<VelocityInfo>,
	pub motion_type: Option<MotionType>,
	pub position_rotation_info: Option<PositionRotationInfo>,
}

impl<R: Read> Deserialize<LE, BEBitReader<R>> for SimplePhysicsConstruction {
	fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self> {
		let is_climbable = reader.read_bit()?;
		let climbing_property = if is_climbable {
			Some(LERead::read(reader)?)
		} else {
			let prop: u32 = LERead::read(reader)?;
			assert_eq!(prop, 0);
			None
		};
		let velocity_info = ReplicaD::deserialize(reader)?;
		let motion_type = ReplicaD::deserialize(reader)?;
		let position_rotation_info = ReplicaD::deserialize(reader)?;
		Ok(Self {
				climbing_property,
				velocity_info,
				motion_type,
				position_rotation_info,
		})
	}
}

impl<'a, W: Write> Serialize<LE, BEBitWriter<W>> for &'a SimplePhysicsConstruction {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		writer.write_bit(self.climbing_property.is_some())?;
		match &self.climbing_property {
			Some(x) => LEWrite::write(writer, x)?,
			None    => LEWrite::write(writer, 0u32)?,
		}
		ReplicaS::serialize(&self.velocity_info, writer)?;
		ReplicaS::serialize(&self.motion_type, writer)?;
		ReplicaS::serialize(&self.position_rotation_info, writer)?;
		Ok(())
	}
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
