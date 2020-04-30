//! All packets an auth server can receive.
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;

use crate::common::{err, LuWStr33, LuWStr41, LuWStr128, LuWStr256, ServiceId};
pub use crate::general::server::GeneralMessage;

rak_server_msg!(LUMessage);

#[derive(Debug)]
#[non_exhaustive]
pub enum LUMessage {
	General(GeneralMessage),
	Auth(AuthMessage),
}

impl<R: LERead> Deserialize<LE, R> for LUMessage
	where        u16: Deserialize<LE, R>,
	  GeneralMessage: Deserialize<LE, R>,
	     AuthMessage: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let service_id: ServiceId = reader.read()?;
		Ok(match service_id {
			ServiceId::General => {
				LUMessage::General(reader.read()?)
			}
			ServiceId::Auth => {
				LUMessage::Auth(reader.read()?)
			}
			_ => {
				return err("invalid service id");
			}
		})
	}
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
			err("invalid auth id")
		}
	}
}

#[derive(Debug)]
pub struct LoginRequest {
	pub username: LuWStr33,
	pub password: LuWStr41,
	pub locale_id: u16,
	pub os_id: u8,
	pub memory_stats: LuWStr256,
	pub video_card_info: LuWStr128,
	pub number_of_processors: u32,
	pub processor_type: u32,
	pub processor_level: u16,
	pub processor_revision: u16,
	pub os_version_info_size: u32,
	pub os_major_version: u32,
	pub os_minor_version: u32,
	pub os_build_number: u32,
	pub os_platform_id: u32,
}

impl<R: LERead> Deserialize<LE, R> for LoginRequest
	where    u8: Deserialize<LE, R>,
	        u16: Deserialize<LE, R>,
	        u32: Deserialize<LE, R>,
	   LuWStr33: Deserialize<LE, R>,
	   LuWStr41: Deserialize<LE, R>,
	  LuWStr128: Deserialize<LE, R>,
	  LuWStr256: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let username             = reader.read()?;
		let password             = reader.read()?;
		let locale_id            = reader.read()?;
		let os_id                = reader.read()?;
		let memory_stats         = reader.read()?;
		let video_card_info      = reader.read()?;
		let number_of_processors = reader.read()?;
		let processor_type       = reader.read()?;
		let processor_level      = reader.read()?;
		let processor_revision   = reader.read()?;
		let os_version_info_size = reader.read()?;
		let os_major_version     = reader.read()?;
		let os_minor_version     = reader.read()?;
		let os_build_number      = reader.read()?;
		let os_platform_id       = reader.read()?;
		Ok(Self {
			username,
			password,
			locale_id,
			os_id,
			memory_stats,
			video_card_info,
			number_of_processors,
			processor_type,
			processor_level,
			processor_revision,
			os_version_info_size,
			os_major_version,
			os_minor_version,
			os_build_number,
			os_platform_id,
		})
	}
}
