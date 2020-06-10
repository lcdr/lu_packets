//! All packets a world server can receive.
use std::io::Read;
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;

use crate::common::{err, ObjId, LuWStr33, LuWStr42, LuStr33, ServiceId, ZoneId};
use crate::chat::server::ChatMessage;
pub use crate::general::server::GeneralMessage;

rak_server_msg!(LuMessage);

#[derive(Debug, Deserialize)]
#[non_exhaustive]
#[repr(u16)]
pub enum LuMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	World(WorldMessage) = ServiceId::World as u16,
}

enum WorldId {
	ClientValidation = 1,
	CharacterListRequest = 2,
	CharacterCreateRequest = 3,
	CharacterLoginRequest = 4,
	CharacterDeleteRequest = 6,
	GeneralChatMessage = 14,
	LevelLoadComplete = 19,
	RouteMessage = 21,
	StringCheck = 25,
	RequestFreeTrialRefresh = 32,
	UgcDownloadFailed = 120,
}

#[derive(Debug)]
pub enum WorldMessage {
	ClientValidation(ClientValidation),
	CharacterListRequest,
	CharacterCreateRequest(CharacterCreateRequest),
	CharacterLoginRequest(CharacterLoginRequest),
	CharacterDeleteRequest(CharacterDeleteRequest),
	GeneralChatMessage(GeneralChatMessage),
	LevelLoadComplete(LevelLoadComplete),
	RouteMessage(RouteMessage),
	StringCheck(StringCheck),
	RequestFreeTrialRefresh,
	UgcDownloadFailed(UgcDownloadFailed),
}

impl<R: LERead> Deserialize<LE, R> for WorldMessage
	where                 u8: Deserialize<LE, R>,
	                     u32: Deserialize<LE, R>,
	        ClientValidation: Deserialize<LE, R>,
	  CharacterCreateRequest: Deserialize<LE, R>,
	   CharacterLoginRequest: Deserialize<LE, R>,
	  CharacterDeleteRequest: Deserialize<LE, R>,
	      GeneralChatMessage: Deserialize<LE, R>,
	       LevelLoadComplete: Deserialize<LE, R>,
	            RouteMessage: Deserialize<LE, R>,
	             StringCheck: Deserialize<LE, R>,
	       UgcDownloadFailed: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let packet_id: u32 = reader.read()?;
		let _padding: u8   = reader.read()?;
		if packet_id == WorldId::ClientValidation as u32 {
			Ok(Self::ClientValidation(reader.read()?))
		} else if packet_id == WorldId::CharacterListRequest as u32 {
			Ok(Self::CharacterListRequest)
		} else if packet_id == WorldId::CharacterCreateRequest as u32 {
			Ok(Self::CharacterCreateRequest(reader.read()?))
		} else if packet_id == WorldId::CharacterLoginRequest as u32 {
			Ok(Self::CharacterLoginRequest(reader.read()?))
		} else if packet_id == WorldId::CharacterDeleteRequest as u32 {
			Ok(Self::CharacterDeleteRequest(reader.read()?))
		} else if packet_id == WorldId::GeneralChatMessage as u32 {
			Ok(Self::GeneralChatMessage(reader.read()?))
		} else if packet_id == WorldId::LevelLoadComplete as u32 {
			Ok(Self::LevelLoadComplete(reader.read()?))
		} else if packet_id == WorldId::RouteMessage as u32 {
			Ok(Self::RouteMessage(reader.read()?))
		} else if packet_id == WorldId::StringCheck as u32 {
			Ok(Self::StringCheck(reader.read()?))
		} else if packet_id == WorldId::RequestFreeTrialRefresh as u32 {
			Ok(Self::RequestFreeTrialRefresh)
		} else if packet_id == WorldId::UgcDownloadFailed as u32 {
			Ok(Self::UgcDownloadFailed(reader.read()?))
		} else {
			err("world id", packet_id)
		}
	}
}

#[derive(Debug)]
pub struct ClientValidation {
	pub username: LuWStr33,
	pub session_key: LuWStr33,
	pub fdb_checksum: [u8; 32],
}

impl<R: Read+LERead> Deserialize<LE, R> for ClientValidation
	where LuWStr33: Deserialize<LE, R>,
	       LuStr33: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let username         = LERead::read(reader)?;
		let session_key      = LERead::read(reader)?;
		let mut fdb_checksum = [0; 32];
		std::io::Read::read(reader, &mut fdb_checksum)?;
		// garbage byte because the devs messed up the null terminator
		let _ : u8           =  LERead::read(reader)?;
		Ok(Self {
			username,
			session_key,
			fdb_checksum,
		})
	}
}

#[derive(Debug)]
pub struct CharacterCreateRequest {
	pub char_name: LuWStr33,
	pub predef_name_ids: (u32, u32, u32),
	pub shirt_color: u32,
	pub pants_color: u32,
	pub hair_style: u32,
	pub hair_color: u32,
	pub eyebrow_style: u32,
	pub eye_style: u32,
	pub mouth_style: u32,
}

impl<R: LERead> Deserialize<LE, R> for CharacterCreateRequest
	where  u8: Deserialize<LE, R>,
	      u32: Deserialize<LE, R>,
	 LuWStr33: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let char_name = reader.read()?;
		let name_id_1 = reader.read()?;
		let name_id_2 = reader.read()?;
		let name_id_3 = reader.read()?;
		let predef_name_ids = (name_id_1, name_id_2, name_id_3);
		let _unused: u8   = reader.read()?;
		let _unused: u32  = reader.read()?;
		let _unused: u32  = reader.read()?;
		let shirt_color   = reader.read()?;
		let _unused: u32  = reader.read()?;
		let pants_color   = reader.read()?;
		let hair_style    = reader.read()?;
		let hair_color    = reader.read()?;
		let _unused: u32  = reader.read()?;
		let _unused: u32  = reader.read()?;
		let eyebrow_style = reader.read()?;
		let eye_style     = reader.read()?;
		let mouth_style   = reader.read()?;
		let _unused: u8  = reader.read()?;

		Ok(Self {
			char_name,
			predef_name_ids,
			shirt_color,
			pants_color,
			hair_style,
			hair_color,
			eyebrow_style,
			eye_style,
			mouth_style,
		})
	}
}

#[derive(Debug, Deserialize)]
pub struct CharacterLoginRequest {
	pub char_id: ObjId,
}

#[derive(Debug, Deserialize)]
pub struct CharacterDeleteRequest {
	pub char_id: ObjId,
}

#[derive(Debug)]
pub struct GeneralChatMessage {
	pub chat_channel: u8, // todo: type?
	pub source_id: u16,
	pub message: String,
}

impl<R: Read+LERead> Deserialize<LE, R> for GeneralChatMessage
	where u8: Deserialize<LE, R>,
	     u16: Deserialize<LE, R>,
	     u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let chat_channel    = LERead::read(reader)?;
		let source_id       = LERead::read(reader)?;
		let string_len: u32 = LERead::read(reader)?;
		let mut string = vec![0; (string_len*2) as usize];
		let mut taken = Read::take(reader, (string_len*2) as u64);
		Read::read(&mut taken, &mut string)?;
		let string_slice: &[u16] = unsafe { std::slice::from_raw_parts(string.as_ptr() as *const u16, string_len as usize - 1) };
		let message = String::from_utf16_lossy(string_slice);

		Ok(Self { chat_channel, source_id, message })
	}
}

#[derive(Debug, Deserialize)]
pub struct LevelLoadComplete {
	pub zone_id: ZoneId,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RouteMessage {
	Chat(ChatMessage),
}

impl<R: LERead> Deserialize<LE, R> for RouteMessage
	where     u32: Deserialize<LE, R>,
	    ServiceId: Deserialize<LE, R>,
	  ChatMessage: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let _packet_size: u32 = reader.read()?;
		let service_id: ServiceId = reader.read()?;
		Ok(match service_id {
			ServiceId::Chat => {
				Self::Chat(reader.read()?)
			}
			_ => {
				return err("route service id", service_id);
			}
		})
	}
}

#[derive(Debug)]
pub struct StringCheck {
	pub chat_mode: u8, // todo: type?
	pub chat_channel: u8, // todo: type?
	pub recipient_name: LuWStr42,
	pub string: String,
}

impl<R: Read+LERead> Deserialize<LE, R> for StringCheck
	where   u8: Deserialize<LE, R>,
	       u16: Deserialize<LE, R>,
	  LuWStr42: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let chat_mode       = LERead::read(reader)?;
		let chat_channel    = LERead::read(reader)?;
		let recipient_name  = LERead::read(reader)?;
		let string_len: u16 = LERead::read(reader)?;
		let mut string = vec![0; (string_len*2) as usize];
		let mut taken = Read::take(reader, (string_len*2) as u64);
		Read::read(&mut taken, &mut string)?;
		let string_slice: &[u16] = unsafe { std::slice::from_raw_parts(string.as_ptr() as *const u16, string_len as usize) };
		let string = String::from_utf16_lossy(string_slice);

		Ok(Self { chat_mode, chat_channel, recipient_name, string })
	}
}

#[derive(Debug, Deserialize)]
#[repr(u32)]
pub enum UgcResType {
	Lxfml,
	Nif,
	Hkx,
	Dds,
}

#[derive(Debug, Deserialize)]
pub struct UgcDownloadFailed {
	pub res_type: UgcResType,
	pub blueprint_id: ObjId,
	pub status_code: u32,
	pub char_id: ObjId,
}
