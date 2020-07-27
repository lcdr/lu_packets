//! All packets a world server can receive.
use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::VariantTests;

use crate::common::{ObjId, LuVarWString, LuWString33, LuWString42, ServiceId};
use crate::chat::server::ChatMessage;
use super::ZoneId;
use super::gm::server::SubjectGameMessage;

pub use crate::general::server::GeneralMessage;

pub type Message = crate::raknet::server::Message<LuMessage>;

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[repr(u16)]
pub enum LuMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	World(WorldMessage) = ServiceId::World as u16,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding=1]
#[repr(u32)]
pub enum WorldMessage {
	ClientValidation(ClientValidation) = 1,
	CharacterListRequest = 2,
	CharacterCreateRequest(CharacterCreateRequest) = 3,
	CharacterLoginRequest(CharacterLoginRequest) = 4,
	SubjectGameMessage(SubjectGameMessage) = 5,
	CharacterDeleteRequest(CharacterDeleteRequest) = 6,
	GeneralChatMessage(GeneralChatMessage) = 14,
	LevelLoadComplete(LevelLoadComplete) = 19,
	RouteMessage(RouteMessage) = 21,
	StringCheck(StringCheck) = 25,
	RequestFreeTrialRefresh = 32,
	UgcDownloadFailed(UgcDownloadFailed) = 120,
}

#[derive(Debug, PartialEq)]
pub struct ClientValidation {
	pub username: LuWString33,
	pub session_key: LuWString33,
	pub fdb_checksum: [u8; 32],
}

impl<R: Read+LERead> Deserialize<LE, R> for ClientValidation
	where   u8: Deserialize<LE, R>,
	  LuWString33: Deserialize<LE, R> {
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

impl<'a, W: Write+LEWrite> Serialize<LE, W> for &'a ClientValidation
	where       u8: Serialize<LE, W>,
	  &'a LuWString33: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()> {
		LEWrite::write(writer, &self.username)?;
		LEWrite::write(writer, &self.session_key)?;
		Write::write(writer, &self.fdb_checksum)?;
		// garbage byte because the devs messed up the null terminator
		LEWrite::write(writer, 0u8)
	}
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding=1]
pub struct CharacterCreateRequest {
	pub char_name: LuWString33,
	pub predef_name_id_1: u32,
	pub predef_name_id_2: u32,
	pub predef_name_id_3: u32,
	#[padding=9]
	pub shirt_color: u32,
	#[padding=4]
	pub pants_color: u32,
	pub hair_style: u32,
	pub hair_color: u32,
	#[padding=8]
	pub eyebrow_style: u32,
	pub eye_style: u32,
	pub mouth_style: u32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterLoginRequest {
	pub char_id: ObjId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterDeleteRequest {
	pub char_id: ObjId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct GeneralChatMessage {
	pub chat_channel: u8, // todo: type?
	pub source_id: u16,
	pub message: LuVarWString<u32>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LevelLoadComplete {
	pub zone_id: ZoneId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[pre_disc_padding=4]
#[repr(u16)]
pub enum RouteMessage {
	Chat(ChatMessage) = ServiceId::Chat as u16,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StringCheck {
	pub chat_mode: u8, // todo: type?
	pub chat_channel: u8, // todo: type?
	pub recipient_name: LuWString42,
	pub string: LuVarWString<u16>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum UgcResType {
	Lxfml,
	Nif,
	Hkx,
	Dds,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct UgcDownloadFailed {
	pub res_type: UgcResType,
	pub blueprint_id: ObjId,
	pub status_code: u32,
	pub char_id: ObjId,
}
