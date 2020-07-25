use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::{FromVariants, VariantTests};

use crate::common::{ObjId, LuString33, LuWString33, LVec};
use super::{Lot, Vector3, ZoneId};
use super::gm::client::SubjectGameMessage;

pub type LuMessage = crate::general::client::LuMessage<ClientMessage>;
pub type Message = crate::raknet::client::Message<LuMessage>;

impl From<ClientMessage> for Message {
	fn from(msg: ClientMessage) -> Self {
		LuMessage::Client(msg).into()
	}
}

#[derive(Debug, Deserialize, PartialEq, Serialize, FromVariants, VariantTests)]
#[non_exhaustive]
#[post_disc_padding=1]
#[repr(u32)]
pub enum ClientMessage {
	LoadStaticZone(LoadStaticZone) = 2,
	CharacterListResponse(CharacterListResponse) = 6,
	CharacterCreateResponse(CharacterCreateResponse) = 7,
	CharacterDeleteResponse(CharacterDeleteResponse) = 11,
	SubjectGameMessage(SubjectGameMessage) = 12,
	TransferToWorld(TransferToWorld) = 14,
	BlueprintLoadItemResponse(BlueprintLoadItemResponse) = 23,
	FriendRequest(FriendRequest) = 27,
	TeamInvite(TeamInvite) = 35,
	MinimumChatModeResponse(MinimumChatModeResponse) = 57,
	MinimumChatModeResponsePrivate(MinimumChatModeResponsePrivate) = 58,
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LoadStaticZone {
	pub zone_id: ZoneId,
	pub map_checksum: u32,
	// editor enabled and editor level, unused
	#[padding=2]
	pub player_position: Vector3,
	pub instance_type: InstanceType,
}

#[derive(Debug, PartialEq)]
pub struct CharacterListResponse {
	pub selected_char: u8,
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharListChar {
	pub obj_id: ObjId,
	#[padding=4]
	pub char_name: LuWString33,
	pub pending_name: LuWString33,
	pub requires_rename: bool,
	pub is_free_trial: bool,
	#[padding=10]
	pub shirt_color: u32,
	#[padding=4]
	pub pants_color: u32,
	pub hair_style: u32,
	pub hair_color: u32,
	#[padding=8]
	pub eyebrow_style: u32,
	pub eye_style: u32,
	pub mouth_style: u32,
	#[padding=4]
	pub last_location: ZoneId,
	#[padding=8]
	pub equipped_items: LVec<Lot, u16>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum CharacterCreateResponse {
	Success,
	GeneralFailure,
	NameNotAllowed,
	PredefinedNameInUse,
	CustomNameInUse,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CharacterDeleteResponse {
	pub success: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TransferToWorld {
	pub redirect_ip: LuString33,
	pub redirect_port: u16,
	pub is_maintenance_transfer: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BlueprintLoadItemResponse {
	pub success: bool,
	pub item_id: ObjId,
	pub dest_item_id: ObjId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FriendRequest {
	pub sender_name: LuWString33,
	pub is_best_friend_request: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TeamInvite {
	pub sender_name: LuWString33,
	pub sender_id: ObjId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MinimumChatModeResponse {
	pub chat_mode: u8, // todo: type?
	pub chat_channel: u8, // todo: type?
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MinimumChatModeResponsePrivate {
	pub chat_mode: u8, // todo: type?
	pub chat_channel: u8, // todo: type?
	pub recipient_name: LuWString33,
	pub recipient_gm_level: u8,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct UpdateFreeTrialStatus {
	pub is_free_trial: bool,
}
