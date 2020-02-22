use std::io::Read;
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;

use crate::common::{err, LuWStr33, LuWStr41, LuWStr128, LuWStr256, ServiceId};

enum MessageId {
	InternalPing = 0,
	ConnectionRequest = 4,
	UserMessage = 83,
}

#[derive(Debug)]
pub enum Message {
	InternalPing(InternalPing),
	ConnectionRequest(ConnectionRequest),
	UserMessage(LUMessage),
}

impl<R: LERead> Deserialize<LE, R> for Message
	where            u8: Deserialize<LE, R>,
	       InternalPing: Deserialize<LE, R>,
	  ConnectionRequest: Deserialize<LE, R>,
	          LUMessage: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let message_id: u8 = LERead::read(reader)?;
		Ok(if message_id == MessageId::InternalPing as u8 {
			Self::InternalPing(LERead::read(reader)?)
		}	else if message_id == MessageId::ConnectionRequest as u8 {
			Self::ConnectionRequest(LERead::read(reader)?)
		} else if message_id == MessageId::UserMessage as u8 {
			Self::UserMessage(LERead::read(reader)?)
		} else {
			return err("invalid message id");
		})
	}
}

#[derive(Debug)]
pub struct InternalPing {
	pub send_time: u32
}

impl<R: LERead> Deserialize<LE, R> for InternalPing
	where u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		Ok(Self { send_time: reader.read()? })
	}
}

#[derive(Debug)]
pub struct ConnectionRequest {
	pub password: Box<[u8]>
}

impl<R: Read> Deserialize<LE, R> for ConnectionRequest {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let mut password = vec![];
		Read::read_to_end(reader, &mut password)?;
		let password = password.into_boxed_slice();
		Ok(Self { password })
	}
}

#[derive(Debug)]
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

enum GeneralId {
	Handshake = 0,
}

#[derive(Debug)]
pub enum GeneralMessage {
	Handshake(Handshake)
}

impl<R: LERead> Deserialize<LE, R> for GeneralMessage
	where    u8: Deserialize<LE, R>,
	        u32: Deserialize<LE, R>,
	  Handshake: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let packet_id: u32 = reader.read()?;
		let _padding: u8   = reader.read()?;
		if packet_id == GeneralId::Handshake as u32 {
			Ok(GeneralMessage::Handshake(reader.read()?))
		} else {
			err("invalid general id")
		}
	}
}

#[derive(Debug)]
pub struct Handshake {
	pub network_version: u32,
	pub service_id: ServiceId,
}

impl<R: LERead> Deserialize<LE, R> for Handshake
	where u8: Deserialize<LE, R>,
	     u16: Deserialize<LE, R>,
	     u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let network_version = reader.read()?;
		let _: u32          = reader.read()?;
		let service_id      = reader.read()?;
		let _: u16          = reader.read()?;
		Ok(Self { network_version, service_id })
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
