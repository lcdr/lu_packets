use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct VendorInfo {
    pub has_standard_items: bool,
    pub has_multicost_items: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct VendorConstruction {
    pub vendor_info: Option<VendorInfo>,
}

impl ComponentConstruction for VendorConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub type VendorSerialization = VendorConstruction;

impl ComponentSerialization for VendorSerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct VendorProtocol;

impl ComponentProtocol for VendorProtocol {
    type Construction = VendorConstruction;
    type Serialization = VendorSerialization;
}
