//! All packets an auth server can receive.
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;

use crate::common::{err, LuWStr33, LuWStr41, LuWStr128, LuWStr256, ServiceId};
pub use crate::general::server::GeneralMessage;

rak_server_msg!(LuMessage);

#[derive(Debug, Deserialize)]
#[non_exhaustive]
#[repr(u16)]
pub enum LuMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	Auth(AuthMessage) = ServiceId::Auth as u16,
}

enum AuthId {
	LoginRequest = 0,
}

#[derive(Debug)]
pub enum AuthMessage {
	LoginRequest(LoginRequest)
}

impl<R: LERead> Deserialize<LE, R> for AuthMessage
	where       u8: Deserialize<LE, R>,
	           u32: Deserialize<LE, R>,
	  LoginRequest: Deserialize<LE, R>, {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let packet_id: u32 = reader.read()?;
		let _padding: u8   = reader.read()?;
		if packet_id == AuthId::LoginRequest as u32 {
			Ok(AuthMessage::LoginRequest(reader.read()?))
		} else {
			err("auth id", packet_id)
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
	pub username: LuWStr33,
	pub password: LuWStr41,
	pub locale_id: u16,
	pub client_os: ClientOs,
	pub computer_stats: ComputerStats,
}

#[derive(Debug, Deserialize)]
#[repr(u8)]
pub enum ClientOs {
	Unknown = 0,
	Windows = 1,
	MacOsX = 2
}

#[derive(Debug, Deserialize)]
pub struct ComputerStats {
	pub memory_stats: LuWStr256,
	pub video_card_info: LuWStr128,
	pub processor_info: ProcessorInfo,
	pub os_info: OsInfo,
}

#[derive(Debug, Deserialize)]
pub struct ProcessorInfo {
	pub number_of_processors: u32,
	pub processor_type: u32,
	pub processor_level: u16,
	pub processor_revision: u16,
}

#[derive(Debug, Deserialize)]
pub struct OsInfo {
	pub os_version_info_size: u32,
	pub major_version: u32,
	pub minor_version: u32,
	pub build_number: u32,
	pub platform_id: u32,
}
