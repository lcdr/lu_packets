//! All packets an auth server can receive.
use endio::{Deserialize, Serialize};
use lu_packets_derive::ServiceMessageD;

use crate::common::{LuWStr33, LuWStr41, LuWStr128, LuWStr256, ServiceId};
pub use crate::general::server::GeneralMessage;

pub type Message = crate::raknet::server::Message<LuMessage>;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
#[repr(u16)]
pub enum LuMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	Auth(AuthMessage) = ServiceId::Auth as u16,
}

#[derive(Debug, ServiceMessageD)]
#[disc_padding=1]
#[repr(u32)]
pub enum AuthMessage {
	LoginRequest(LoginRequest)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
	pub username: LuWStr33,
	pub password: LuWStr41,
	pub locale_id: u16,
	pub client_os: ClientOs,
	pub computer_stats: ComputerStats,
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum ClientOs {
	Unknown,
	Windows,
	MacOs,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComputerStats {
	pub memory_stats: LuWStr256,
	pub video_card_info: LuWStr128,
	pub processor_info: ProcessorInfo,
	pub os_info: OsInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessorInfo {
	pub number_of_processors: u32,
	pub processor_type: u32,
	pub processor_level: u16,
	pub processor_revision: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OsInfo {
	pub os_version_info_size: u32,
	pub major_version: u32,
	pub minor_version: u32,
	pub build_number: u32,
	pub platform_id: u32,
}
