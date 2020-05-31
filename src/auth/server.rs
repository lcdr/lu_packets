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
				return err("service id", service_id);
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
			err("auth id", packet_id)
		}
	}
}

#[derive(Debug)]
pub struct LoginRequest {
	pub username: LuWStr33,
	pub password: LuWStr41,
	pub locale_id: u16,
	pub client_os: ClientOs,
	pub computer_stats: ComputerStats,
}

impl<R: LERead> Deserialize<LE, R> for LoginRequest
	where  ClientOs: Deserialize<LE, R>,
	            u16: Deserialize<LE, R>,
	       LuWStr33: Deserialize<LE, R>,
	       LuWStr41: Deserialize<LE, R>,
	  ComputerStats: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let username       = reader.read()?;
		let password       = reader.read()?;
		let locale_id      = reader.read()?;
		let client_os      = reader.read()?;
		let computer_stats = reader.read()?;
		Ok(Self {
			username,
			password,
			locale_id,
			client_os,
			computer_stats,
		})
	}
}

#[derive(Debug)]
pub enum ClientOs {
	Unknown = 0,
	Windows = 1,
	MacOsX = 2
}

impl<R: LERead> Deserialize<LE, R> for ClientOs
	where u8: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let client_os: u8 = reader.read()?;
		Ok(if client_os == ClientOs::Unknown as u8 {
			ClientOs::Unknown
		} else if client_os == ClientOs::Windows as u8 {
			ClientOs::Windows
		} else if client_os == ClientOs::MacOsX as u8 {
			ClientOs::MacOsX
		} else {
			return err("client os", client_os);
		})
	}
}

#[derive(Debug)]
pub struct ComputerStats {
	pub memory_stats: LuWStr256,
	pub video_card_info: LuWStr128,
	pub processor_info: ProcessorInfo,
	pub os_info: OsInfo,
}

impl<R: LERead> Deserialize<LE, R> for ComputerStats
	where LuWStr128: Deserialize<LE, R>,
	      LuWStr256: Deserialize<LE, R>,
	  ProcessorInfo: Deserialize<LE, R>,
	         OsInfo: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let memory_stats    = reader.read()?;
		let video_card_info = reader.read()?;
		let processor_info  = reader.read()?;
		let os_info         = reader.read()?;
		Ok(Self {
			memory_stats,
			video_card_info,
			processor_info,
			os_info,
		})
	}
}

#[derive(Debug)]
pub struct ProcessorInfo {
	pub number_of_processors: u32,
	pub processor_type: u32,
	pub processor_level: u16,
	pub processor_revision: u16,
}

impl<R: LERead> Deserialize<LE, R> for ProcessorInfo
	where   u16: Deserialize<LE, R>,
	        u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let number_of_processors = reader.read()?;
		let processor_type       = reader.read()?;
		let processor_level      = reader.read()?;
		let processor_revision   = reader.read()?;
		Ok(Self {
			number_of_processors,
			processor_type,
			processor_level,
			processor_revision,
		})
	}
}

#[derive(Debug)]
pub struct OsInfo {
	pub os_version_info_size: u32,
	pub major_version: u32,
	pub minor_version: u32,
	pub build_number: u32,
	pub platform_id: u32,
}

impl<R: LERead> Deserialize<LE, R> for OsInfo
	where u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let os_version_info_size = reader.read()?;
		let major_version     = reader.read()?;
		let minor_version     = reader.read()?;
		let build_number      = reader.read()?;
		let platform_id       = reader.read()?;
		Ok(Self {
			os_version_info_size,
			major_version,
			minor_version,
			build_number,
			platform_id,
		})
	}
}
