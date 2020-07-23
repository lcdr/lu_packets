//! All packets an auth server can receive.
use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;

use crate::common::{LuWString33, LuWString41, LuWString128, LuWString256, ServiceId};
pub use crate::general::server::GeneralMessage;

pub type Message = crate::raknet::server::Message<LuMessage>;

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[non_exhaustive]
#[repr(u16)]
pub enum LuMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	Auth(AuthMessage) = ServiceId::Auth as u16,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding=1]
#[repr(u32)]
pub enum AuthMessage {
	LoginRequest(LoginRequest)
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LoginRequest {
	pub username: LuWString33,
	pub password: LuWString41,
	pub locale_id: u16,
	pub client_os: ClientOs,
	pub computer_stats: ComputerStats,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum ClientOs {
	Unknown,
	Windows,
	MacOs,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ComputerStats {
	pub memory_stats: LuWString256,
	pub video_card_info: LuWString128,
	pub processor_info: ProcessorInfo,
	pub os_info: OsInfo,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ProcessorInfo {
	pub number_of_processors: u32,
	pub processor_type: u32,
	pub processor_level: u16,
	pub processor_revision: u16,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OsInfo {
	pub os_version_info_size: u32,
	pub major_version: u32,
	pub minor_version: u32,
	pub build_number: u32,
	pub platform_id: u32,
}
