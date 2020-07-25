use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;

use crate::common::ServiceId;

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding=1]
#[repr(u32)]
pub enum GeneralMessage {
	Handshake(Handshake)
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding=33]
pub struct Handshake {
	pub network_version: u32,
	#[padding=4]
	pub service_id: ServiceId,
	#[padding=2]
	pub process_id: u32,
	pub port: u16,
}
