use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentSerialization};
use super::vendor::VendorInfo;

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct AchievementVendorConstruction {
	pub vendor_info: Option<VendorInfo>,
}

impl ComponentConstruction for AchievementVendorConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub type AchievementVendorSerialization = AchievementVendorConstruction;

impl ComponentSerialization for AchievementVendorSerialization {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
