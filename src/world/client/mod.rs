//! Client-received world messages.
use std::io::{Error, ErrorKind::InvalidData, Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::{FromVariants, VariantTests};

use crate::chat::ChatChannel;
use crate::chat::client::ChatMessage;
use crate::common::{ObjId, LuString33, LuWString33, LuWString42, LVec, ServiceId};
use crate::general::client::{DisconnectNotify, Handshake, GeneralMessage};
use super::{Lot, lnv::LuNameValue, Vector3, ZoneId};
use super::gm::client::SubjectGameMessage;

/// All messages that can be received by a client from a world server.
pub type Message = crate::raknet::client::Message<LuMessage>;

/// All client-received LU messages from a world server.
#[derive(Debug, Deserialize, FromVariants, PartialEq, Serialize, VariantTests)]
#[repr(u16)]
pub enum LuMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	Chat(ChatMessage) = ServiceId::Chat as u16,
	Client(ClientMessage) = ServiceId::Client as u16,
}

impl From<LuMessage> for Message {
	fn from(msg: LuMessage) -> Self {
		Message::UserMessage(msg)
	}
}

impl From<Handshake> for Message {
	fn from(msg: Handshake) -> Self {
		GeneralMessage::Handshake(msg).into()
	}
}

impl From<DisconnectNotify> for Message {
	fn from(msg: DisconnectNotify) -> Self {
		GeneralMessage::DisconnectNotify(msg).into()
	}
}

/// All client-received world messages.
#[derive(Debug, Deserialize, PartialEq, Serialize, FromVariants, VariantTests)]
#[non_exhaustive]
#[post_disc_padding=1]
#[repr(u32)]
pub enum ClientMessage {
	LoadStaticZone(LoadStaticZone) = 2,
	CreateCharacter(CreateCharacter) = 4,
	CharacterListResponse(CharacterListResponse) = 6,
	CharacterCreateResponse(CharacterCreateResponse) = 7,
	CharacterDeleteResponse(CharacterDeleteResponse) = 11,
	SubjectGameMessage(SubjectGameMessage) = 12,
	TransferToWorld(TransferToWorld) = 14,
	BlueprintLoadItemResponse(BlueprintLoadItemResponse) = 23,
	AddFriendRequest(AddFriendRequest) = 27,
	AddFriendResponse(AddFriendResponse) = 28,
	GetFriendsListResponse(GetFriendsListResponse) = 30,
	FriendUpdateNotify(FriendUpdateNotify) = 31,
	GetIgnoreListResponse(GetIgnoreListResponse) = 34,
	TeamInvite(TeamInvite) = 35,
	MinimumChatModeResponse(MinimumChatModeResponse) = 57,
	MinimumChatModeResponsePrivate(MinimumChatModeResponsePrivate) = 58,
	ChatModerationString(ChatModerationString) = 59,
	UpdateFreeTrialStatus(UpdateFreeTrialStatus) = 62,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum InstanceType {
	Public,
	Single,
	Team,
	Guild,
	Match,
}

/**
	Tells the client to load a zone.

	### Trigger
	May be sent at any time. However, in a typical server instance architecture, this message will usually be sent as the first message directly after the client has validated itself with [`ClientValidation`](super::server::ClientValidation).

	### Handling
	Load the zone specified in [`zone_id`](Self::zone_id), whatever that may entail for your client implementation.

	### Response
	Respond with [`LevelLoadComplete`](super::server::LevelLoadComplete) once you're done loading.

	### Notes
	Server instances are usually statically assigned to host a "parallel universe" of a certain zone (world), which means that this message will be sent directly after client validation. However, other instance architectures are theoretically possible:

	- Dynamic changing of the instance's zone, in which case additional [`LoadStaticZone`] messages could be sent (when the zone is changed).

	- Shared/overlapping instances, where the instance connection changes as the player moves around in the world, or where instances take over from others (e.g. in the event of a reboot), with mobs and all other state being carried over. In this case the client would be instructed to connect to the new instance via [`TransferToWorld`], but would not receive a [`LoadStaticZone`] afterwards. If done correctly, the player wouldn't even notice the transfer at all.

	However, these are quite advanced architectures, and for now it is unlikely that any server project will actually pull these off.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LoadStaticZone {
	/// ID of the zone to be loaded.
	pub zone_id: ZoneId,
	/// Checksum on the map on the server side. The original LU client will refuse to load any map where the client side checksum doesn't match the server checksum, to prevent inconsistencies and cheating.
	pub map_checksum: u32,
	// editor enabled and editor level, unused
	#[padding=2]
	/// The position of the player in the new world, likely used to be able to load the right part of the world.
	pub player_position: Vector3,
	/// The instance type of the zone being loaded.
	pub instance_type: InstanceType,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CreateCharacter {
	pub data: LuNameValue,
}

/**
	Provides the list of characters of the client's account.

	### Trigger
	Receipt of [`CharacterListRequest`](super::server::WorldMessage::CharacterListRequest). Also sent in response to [`CharacterCreateRequest`](super::server::CharacterCreateRequest) after [`CharacterCreateResponse`] if the creation is successful.

	### Handling
	Display the characters to the user for selection.

	### Response
	None.

	### Notes
	The LU client can't handle sending more than four characters.
*/
#[derive(Debug, PartialEq)]
pub struct CharacterListResponse {
	/// Index into the list of characters below, specifying which character was used last.
	pub selected_char: u8,
	/// The list of characters.
	pub chars: Vec<CharListChar>,
}

impl<R: LERead> Deserialize<LE, R> for CharacterListResponse
	where       u8: Deserialize<LE, R>,
	  CharListChar: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self>	{
		let len: u8 = reader.read()?;
		let selected_char = reader.read()?;
		let mut chars = Vec::with_capacity(len as usize);
		for _ in 0..len {
			chars.push(reader.read()?);
		}
		Ok(Self { selected_char, chars } )
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a CharacterListResponse
	where           u8: Serialize<LE, W>,
	  &'a CharListChar: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.chars.len() as u8)?;
		writer.write(self.selected_char)?;
		for chr in self.chars.iter() {
			writer.write(chr)?;
		}
		Ok(())
	}
}

/// A character from the [`CharacterListResponse`] message.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharListChar {
	pub obj_id: ObjId,
	#[padding=4]
	pub char_name: LuWString33,
	pub pending_name: LuWString33,
	pub requires_rename: bool,
	pub is_free_trial: bool,
	#[padding=10]
	pub torso_color: u32,
	#[padding=4]
	pub legs_color: u32,
	pub hair_style: u32,
	pub hair_color: u32,
	#[padding=8]
	pub eyebrows_style: u32,
	pub eyes_style: u32,
	pub mouth_style: u32,
	#[padding=4]
	pub last_location: ZoneId,
	#[padding=8]
	pub equipped_items: LVec<u16, Lot>,
}

/**
	Reports the result of a character create request.

	### Trigger
	Receipt of [`CharacterCreateRequest`](super::server::CharacterCreateRequest).

	### Handling
	If the variant is not [`Success`](CharacterCreateResponse::Success), display an appropriate error message and let the user try again. If successful, wait for the updated [`CharacterListResponse`] packet to arrive and display the new character list.

	### Response
	None.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum CharacterCreateResponse {
	/// The character has been successfully created.
	Success,
	/// Something went wrong during creation.
	GeneralFailure,
	/// The selected name is not allowed by the name moderation policy.
	NameNotAllowed,
	/// The ThreePartName is already in use.
	PredefinedNameInUse,
	/// The custom name is already in use.
	CustomNameInUse,
}

/**
	Reports the result of a character delete request.

	### Trigger
	Receipt of [`CharacterDeleteRequest`](super::server::CharacterDeleteRequest).

	### Handling
	Delete the character locally if [`success`](Self::success) is `true`, else display an error message and keep the character.

	### Response
	None.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterDeleteResponse {
	/// Whether the deletion was successful.
	pub success: bool,
}

/**
	Tells the client to open a connection to another server instance.

	### Trigger
	The server can send this at any time, but typically does when a launchpad or command is used to go to another world. Other reasons can include the instance shutting down, or exceeding its player limit.

	### Response
	Close the connection after the connection to the other instance has been established.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TransferToWorld {
	/// The host to connect to.
	pub redirect_ip: LuString33,
	/// The port to connect to.
	pub redirect_port: u16,
	/// If this is `true`, the original LU client displays a "Mythran dimensional shift succeeded" announcement.
	pub is_maintenance_transfer: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BlueprintLoadItemResponse {
	pub success: bool,
	pub item_id: ObjId,
	pub dest_item_id: ObjId,
}

/**
	Informs the client that another player has asked them to be their friend.

	### Trigger
	Receipt of `ChatMessage::AddFriendRequest` (todo). Note that friend requests should be supported even if the recipient is on another instance, so a relay infrastructure like a chat server is necessary and needs to be accounted for.

	### Handling
	Display a dialog to the player asking them whether to accept or deny the request.

	### Response
	Respond with [`AddFriendResponse`](crate::chat::server::AddFriendResponse) once the user has made their choice.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AddFriendRequest {
	/// Name of the requesting character.
	pub sender_name: LuWString33,
	/// Whether the request is asking to be best friends instead of just normal friends.
	pub is_best_friend_request: bool,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum AddFriendResponseType {
	Accepted {
		is_online: bool,
		sender_id: ObjId,
		zone_id: ZoneId,
		is_best_friend: bool,
		is_free_trial: bool,
	},
	AlreadyFriend {
		is_best_friend: bool,
	},
	InvalidCharacter,
	GeneralError {
		is_best_friend: bool,
	},
	YourFriendListFull,
	TheirFriendListFull,
	Declined,
	Busy,
	NotOnline,
	WaitingApproval,
	Mythran,
	Cancelled,
	FriendIsFreeTrial,
}

#[derive(Debug, PartialEq)]
pub struct AddFriendResponse {
	char_name: LuWString33,
	response_type: AddFriendResponseType,
}

impl<R: Read> Deserialize<LE, R> for AddFriendResponse {
	fn deserialize(reader: &mut R) -> Res<Self>	{
		let disc: u8       = LERead::read(reader)?;
		let is_online      = LERead::read(reader)?;
		let char_name      = LERead::read(reader)?;
		let sender_id      = LERead::read(reader)?;
		let zone_id        = LERead::read(reader)?;
		let is_best_friend = LERead::read(reader)?;
		let is_free_trial  = LERead::read(reader)?;
		Ok(Self { char_name, response_type:
			match disc {
				0  => AddFriendResponseType::Accepted { is_online, sender_id, zone_id, is_best_friend, is_free_trial },
				1  => AddFriendResponseType::AlreadyFriend { is_best_friend },
				2  => AddFriendResponseType::InvalidCharacter,
				3  => AddFriendResponseType::GeneralError { is_best_friend },
				4  => AddFriendResponseType::YourFriendListFull,
				5  => AddFriendResponseType::TheirFriendListFull,
				6  => AddFriendResponseType::Declined,
				7  => AddFriendResponseType::Busy,
				8  => AddFriendResponseType::NotOnline,
				9  => AddFriendResponseType::WaitingApproval,
				10 => AddFriendResponseType::Mythran,
				11 => AddFriendResponseType::Cancelled,
				12 => AddFriendResponseType::FriendIsFreeTrial,
				_  => { return Err(Error::new(InvalidData, "invalid discriminant for AddFriendResponseType")) }
			}
		})
	}
}

impl<'a, W: Write> Serialize<LE, W> for &'a AddFriendResponse {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		let disc = unsafe { *(&self.response_type as *const AddFriendResponseType as *const u8) };
		LEWrite::write(writer, disc)?;
		let mut is_online_x = &false;
		let mut sender_id_x = &0;
		let mut zone_id_x = &ZoneId { map_id: 0, instance_id: 0, clone_id: 0 };
		let mut is_best_friend_x = &false;
		let mut is_free_trial_x = &false;
		match &self.response_type {
			AddFriendResponseType::Accepted { is_online, sender_id, zone_id, is_best_friend, is_free_trial } => {
				is_online_x = is_online;
				sender_id_x = sender_id;
				zone_id_x = zone_id;
				is_best_friend_x = is_best_friend;
				is_free_trial_x = is_free_trial;
			}
			AddFriendResponseType::AlreadyFriend { is_best_friend } => {
				is_best_friend_x = is_best_friend;
			}
			AddFriendResponseType::GeneralError { is_best_friend } => {
				is_best_friend_x = is_best_friend;
			}
			_ => {},
		}
		LEWrite::write(writer, is_online_x)?;
		LEWrite::write(writer, &self.char_name)?;
		LEWrite::write(writer, sender_id_x)?;
		LEWrite::write(writer, zone_id_x)?;
		LEWrite::write(writer, is_best_friend_x)?;
		LEWrite::write(writer, is_free_trial_x)?;
		Ok(())
	}
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding=6]
pub struct FriendState {
	is_online: bool,
	is_best_friend: bool,
	is_free_trial: bool,
	#[padding=5]
	location: ZoneId,
	object_id: ObjId,
	char_name: LuWString33,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
#[post_disc_padding=2]
pub enum GetFriendsListResponse {
	Ok(LVec<u16, FriendState>),
	GeneralError,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum FriendUpdateType {
	Logout,
	Login,
	Transfer,
	FreeTrialChange,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FriendUpdateNotify {
	update_type: FriendUpdateType,
	char_name: LuWString33,
	zone_id: ZoneId,
	is_best_friend: bool,
	is_free_trial: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding=6]
pub struct IgnoreState {
	object_id: ObjId,
	char_name: LuWString33,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
#[post_disc_padding=2]
pub enum GetIgnoreListResponse {
	Ok(LVec<u16, IgnoreState>),
	GeneralError,
}

/**
	Informs the client that another player has asked them to be their friend.

	### Trigger
	Receipt of `ChatMessage::TeamInvite` (todo). Note that team invites should be supported even if the recipient is on another instance, so a relay infrastructure like a chat server is necessary and needs to be accounted for.

	### Handling
	Display a dialog to the player asking them whether to accept or deny the request.

	### Response
	Respond with [`TeamInviteResponse`](crate::chat::server::TeamInviteResponse) once the user has made their choice.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TeamInvite {
	/// Name of the requesting character.
	pub sender_name: LuWString33,
	/// Object ID of the requesting character.
	pub sender_id: ObjId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MinimumChatModeResponse {
	pub chat_mode: u8, // todo: type?
	pub chat_channel: ChatChannel,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MinimumChatModeResponsePrivate {
	pub chat_mode: u8, // todo: type?
	pub chat_channel: ChatChannel,
	pub recipient_name: LuWString33,
	pub recipient_gm_level: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModerationSpan {
	pub start_index: u8,
	pub length: u8,
}

#[derive(Debug, PartialEq)]
pub struct ChatModerationString {
	//#[padding=2]
	pub request_id: u8,
	pub chat_mode: u8, // todo: type
	pub whisper_name: LuWString42,
	pub spans: Vec<ModerationSpan>,
}

impl<R: Read+LERead> Deserialize<LE, R> for ChatModerationString {
	fn deserialize(reader: &mut R) -> Res<Self>	{
		let _string_okay: bool = LERead::read(reader)?;
		let _source_id: u16 = LERead::read(reader)?; // unused
		let request_id = LERead::read(reader)?;
		let chat_mode = LERead::read(reader)?;
		let whisper_name = LERead::read(reader)?;
		let mut spans = vec![];
		let mut i = 0;
		loop {
			if i > 63 {
				break;
			}
			let start_index = LERead::read(reader)?;
			let length = LERead::read(reader)?;
			if length != 0 {
				spans.push(ModerationSpan { start_index, length });
			}
			i += 1;
		}
		Ok(Self { request_id, chat_mode, whisper_name, spans })
	}
}

impl<'a, W: Write+LEWrite> Serialize<LE, W> for &'a ChatModerationString {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		LEWrite::write(writer, self.spans.is_empty())?;
		LEWrite::write(writer, 0u16)?;
		LEWrite::write(writer, self.request_id)?;
		LEWrite::write(writer, self.chat_mode)?;
		LEWrite::write(writer, &self.whisper_name)?;
		if self.spans.len() > 64 {
			return Err(Error::new(InvalidData, "spans longer than 64"));
		}
		for span in &self.spans {
			LEWrite::write(writer, span.start_index)?;
			LEWrite::write(writer, span.length)?;
		}
		for _ in self.spans.len()..64 {
			Write::write(writer, &[0; 2])?;
		}
		Ok(())
	}
}

/**
	Notifies the client that its free trial status has changed.

	### Trigger
	Sent by the server when the status changes.

	### Handling
	Display appropriate UI, celebration, etc.

	### Response
	None.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct UpdateFreeTrialStatus {
	/// Whether the player is on free trial.
	pub is_free_trial: bool,
}
