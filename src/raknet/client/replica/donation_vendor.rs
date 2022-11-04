use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::vendor::VendorInfo;
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct DonationVendorInfo {
    pub percent_complete: f32,
    pub total_donated: u32,
    pub total_remaining: u32,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct DonationVendorConstruction {
    pub vendor_info: Option<VendorInfo>,
    pub donation_vendor_info: Option<DonationVendorInfo>,
}

impl ComponentConstruction for DonationVendorConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type DonationVendorSerialization = DonationVendorConstruction;

impl ComponentSerialization for DonationVendorSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct DonationVendorProtocol;

impl ComponentProtocol for DonationVendorProtocol {
    type Construction = DonationVendorConstruction;
    type Serialization = DonationVendorSerialization;
}
