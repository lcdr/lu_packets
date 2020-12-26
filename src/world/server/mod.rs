//! Server-received world messages.
pub mod mail;

use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use endio::LittleEndian as LE;
use endio_bit::{BEBitReader, BEBitWriter};
use lu_packets_derive::VariantTests;

use crate::common::{ObjId, LuVarWString, LuWString33, LuWString42, ServiceId};
use crate::chat::ChatChannel;
use crate::chat::server::ChatMessage;
use crate::raknet::client::replica::controllable_physics::FrameStats;
use super::ZoneId;
use super::gm::server::SubjectGameMessage;
use self::mail::Mail;

pub use crate::general::server::GeneralMessage;

/// All messages that can be received by a world server.
pub type Message = crate::raknet::server::Message<LuMessage>;

/// All LU messages that can be received by a world server.
#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[repr(u16)]
pub enum LuMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	World(WorldMessage) = ServiceId::World as u16,
}

/// All server-received world messages.
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
	PositionUpdate(PositionUpdate) = 22,
	Mail(Mail) = 23,
	StringCheck(StringCheck) = 25,
	RequestFreeTrialRefresh = 32,
	Top5IssuesRequest(Top5IssuesRequest) = 91,
	UgcDownloadFailed(UgcDownloadFailed) = 120,
}

/**
	Provides session info for authentication.

	### Trigger
	Receipt of [Server handshake](crate::general::client::Handshake).

	### Handling
	Verify with your auth server that the `(username, session_key)` combination is valid. If not, immediately disconnect the client, ideally with a [`DisconnectNotify::InvalidSessionKey`](crate::general::client::DisconnectNotify::InvalidSessionKey).

	If you are concerned about players modding their client DB, also check the [`fdb_checksum`](Self::fdb_checksum). Note that players can still change their client to send a fake checksum, but this requires exe modding, which most players are presumably not familiar with.

	If all validation checks pass, store the connection -> username association, as this is the only packet that references the username.

	### Response
	The client does not require a fixed response to this packet. However, world servers (with the exception of a dedicated char server) will usually want to respond to this with [`LoadStaticZone`](super::client::LoadStaticZone).

	### Notes
	**Important**: Do **not** handle any other packets from clients that have not yet been validated. Handling other packets before validation can lead to errors because the connection has not yet been associated with a username, and can lead to security vulnerabilities if session keys are not validated properly.
*/
#[derive(Debug, PartialEq)]
pub struct ClientValidation {
	/// Account username.
	pub username: LuWString33,
	/// [Session key from auth's login response](crate::auth::client::LoginResponse::Ok::session_key).
	pub session_key: LuWString33,
	/// MD5 hash of null-terminated cdclient.fdb file contents.
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

/**
	Requests a new character to be created.

	### Trigger
	The player creating a new character and submitting it to the server.

	### Handling
	Check if the predefined name is available. If the custom name is set, check if it is available as well. If your server has a name policy, check if the custom name is known to be unacceptable.

	If all checks pass, create the character and save it to the database using the information specified in this message.

	### Response
	Respond with [`CharacterCreateResponse`](super::client::CharacterCreateResponse), using the appropriate variant to indicate the result. If the character creation is successful, additionally send a [`CharacterListResponse`](super::client::CharacterListResponse) afterwards with the new character included.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding=1]
pub struct CharacterCreateRequest {
	/// The custom name, or blank if the predefined name is to be used.
	pub char_name: LuWString33,
	/// First part of the predefined name.
	pub predef_name_id_1: u32, // todo: enum
	/// Second part of the predefined name.
	pub predef_name_id_2: u32, // todo: enum
	/// Third part of the predefined name.
	pub predef_name_id_3: u32, // todo: enum
	#[padding=9]
	/// Chosen torso color.
	pub torso_color: u32, // todo: enum
	#[padding=4]
	/// Chosen legs color.
	pub legs_color: u32, // todo: enum
	/// Chosen hair style.
	pub hair_style: u32, // todo: enum
	/// Chosen hair color.
	pub hair_color: u32, // todo: enum
	#[padding=8]
	/// Chosen eyebrow style.
	pub eyebrows_style: u32, // todo: enum
	/// Chosen eye style.
	pub eyes_style: u32, // todo: enum
	/// Chosen mouth style.
	pub mouth_style: u32, // todo: enum
}

/**
	Indicates that the player has chosen a character to play with.

	### Trigger
	The player selecting the character in the character selection screen and pressing play.

	### Handling
	Do what's necessary to let the character join the world it was last in. In the common case of the world server instance being different from the current instance, redirect the client to the world instance.

	### Response
	Respond with [`LoadStaticZone`](super::client::LoadStaticZone) if you're not switching instances, or [`TransferToWorld`](super::client::TransferToWorld) if you do.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterLoginRequest {
	/// The object ID of the chosen character.
	pub char_id: ObjId,
}

/**
	Requests a character to be deleted.

	### Trigger
	The player deleting the character and confirming with their password.

	### Handling
	Delete the character from the database, with appropriate cascading deletes, such as deleting the characters from any friends lists they're in.

	### Response
	Respond with [`CharacterDeleteResponse`](super::client::CharacterDeleteResponse) indicating whether deletion was successful.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterDeleteRequest {
	/// The object ID of the chosen character.
	pub char_id: ObjId,
}

#[derive(Debug, PartialEq)]
pub struct GeneralChatMessage {
	pub chat_channel: ChatChannel,
	pub source_id: u16,
	pub message: LuVarWString<u32>,
}

impl<R: Read+LERead> Deserialize<LE, R> for GeneralChatMessage
	where   u8: Deserialize<LE, R>,
	  LuWString33: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let chat_channel = LERead::read(reader)?;
		let source_id    = LERead::read(reader)?;
		let mut str_len: u32 = LERead::read(reader)?;
		str_len -= 1;
		let message = LuVarWString::deser_content(reader, str_len)?;
		let _: u16       = LERead::read(reader)?;
		Ok(Self {
			chat_channel,
			source_id,
			message,
		})
	}
}

impl<'a, W: Write+LEWrite> Serialize<LE, W> for &'a GeneralChatMessage {
	fn serialize(self, writer: &mut W) -> Res<()> {
		LEWrite::write(writer, &self.chat_channel)?;
		LEWrite::write(writer, self.source_id)?;
		let mut str_len = self.message.len();
		str_len += 1;
		LEWrite::write(writer, str_len as u32)?;
		self.message.ser_content(writer)?;
		LEWrite::write(writer, 0u16)
	}
}

/**
	Reports to the server that client-side loading has finished.

	### Trigger
	The client finishing a zone load initiated by [`LoadStaticZone`](super::client::LoadStaticZone).

	### Handling / Response
	Respond with [`CreateCharacter`](super::client::CreateCharacter) containing details about the player's character. Add the client to your server's replica manager, so that existing objects in range are replicated using [`ReplicaConstruction`](crate::raknet::client::replica::ReplicaConstruction). Create the character's replica object and and let the replica manager broadcast its construction to all clients in range. Finally, send [`ServerDoneLoadingAllObjects`](crate::world::gm::client::GameMessage::ServerDoneLoadingAllObjects) from the character object to the client.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LevelLoadComplete {
	/// The ID of the zone that was loaded. Servers should not trust this, as a player could use it to get into zones they don't belong.
	pub zone_id: ZoneId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[pre_disc_padding=4]
#[repr(u16)]
pub enum RouteMessage {
	Chat(ChatMessage) = ServiceId::Chat as u16,
}

#[derive(Debug, PartialEq)]
pub struct PositionUpdate {
	frame_stats: FrameStats,
}

impl<R: Read> Deserialize<LE, R> for PositionUpdate {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let mut reader = BEBitReader::new(reader);
		let frame_stats = LERead::read(&mut reader)?;
		Ok(Self { frame_stats })
	}
}

impl<'a, W: Write> Serialize<LE, W> for &'a PositionUpdate {
	fn serialize(self, writer: &mut W) -> Res<()> {
		let mut writer = BEBitWriter::new(writer);
		LEWrite::write(&mut writer, &self.frame_stats)
	}
}

/**
	Asks the server whether a string the player entered is acceptable.

	### Trigger
	The player entering a string in the chat box. This message is sent as the player is typing, before pressing enter.

	### Handling
	Check whether the [`string`](Self::string) is acceptable per the server's moderation policy, taking into account the player's chat mode and best friend status with possible recipient.

	### Response
	Respond with [`ChatModerationString`](super::client::ChatModerationString), indicating whether the string is ok, or if not, the spans that are not acceptable.

	### Notes
	This message is only for quick player feedback on acceptability. Final string submissions by the player will be sent in different messages (e.g. [`GeneralChatMessage`] or `Mail` (todo)). Those messages will need to be checked for moderation as well. This means that there's no harm in trusting the client to provide accurate context ([`chat_mode`](Self::chat_mode), [`recipient_name`](Self::recipient_name) in this message.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StringCheck {
	pub chat_mode: u8, // todo: type?
	pub request_id: u8,
	pub recipient_name: LuWString42,
	/// The string to be checked.
	pub string: LuVarWString<u16>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum Language {
	en_US,
	pl_US,
	de_DE,
	en_GB,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Top5IssuesRequest {
	language: Language,
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
