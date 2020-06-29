mod gm;

use std::io::Read;
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::FromVariants;

use crate::common::{ObjId, LuStr33, LuWStr33};
use super::{Vector3, ZoneId};
use self::gm::SubjectGameMessage;

pub type LuMessage = crate::general::client::LuMessage<ClientMessage>;
pub type Message = crate::raknet::client::Message<LuMessage>;

impl From<ClientMessage> for Message {
	fn from(msg: ClientMessage) -> Self {
		LuMessage::Client(msg).into()
	}
}

#[derive(Debug, Deserialize, Serialize, FromVariants)]
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

#[derive(Debug, Deserialize, Serialize)]
#[repr(u32)]
pub enum InstanceType {
	Public,
	Single,
	Team,
	Guild,
	Match,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoadStaticZone {
	pub zone_id: ZoneId,
	pub map_checksum: u32,
	// editor enabled and editor level, unused
	#[padding=2]
	pub player_position: Vector3,
	pub instance_type: InstanceType,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CharListChar {
	pub obj_id: u64,
	pub char_name: LuWStr33,
	pub pending_name: LuWStr33,
	pub requires_rename: bool,
	pub is_free_trial: bool,
	pub shirt_color: u32,
	pub pants_color: u32,
	pub hair_style: u32,
	pub hair_color: u32,
	pub eyebrow_style: u32,
	pub eye_style: u32,
	pub mouth_style: u32,
	pub last_location: ZoneId,
}

impl<R: Read+LERead> Deserialize<LE, R> for CharListChar {
	fn deserialize(reader: &mut R) -> Res<Self>	{
		let obj_id          = LERead::read(reader)?;
		let _: u32          = LERead::read(reader)?;
		let char_name       = LERead::read(reader)?;
		let pending_name    = LERead::read(reader)?;
		let requires_rename = LERead::read(reader)?;
		let is_free_trial   = LERead::read(reader)?;
		let mut unused = [0; 10];
		Read::read(reader, &mut unused)?;
		let shirt_color     = LERead::read(reader)?;
		let mut unused = [0; 4];
		Read::read(reader, &mut unused)?;
		let pants_color = LERead::read(reader)?;
		let hair_style = LERead::read(reader)?;
		let hair_color = LERead::read(reader)?;
		let mut unused = [0; 8];
		Read::read(reader, &mut unused)?;
		let eyebrow_style = LERead::read(reader)?;
		let eye_style = LERead::read(reader)?;
		let mouth_style = LERead::read(reader)?;
		let mut unused = [0; 4];
		Read::read(reader, &mut unused)?;
		let last_location = LERead::read(reader)?;
		let mut unused = [0; 8];
		Read::read(reader, &mut unused)?;
		let items_len: u16 = LERead::read(reader)?;
		assert_eq!(items_len, 0); // todo
		Ok(Self {
			obj_id, char_name, pending_name, requires_rename,
			is_free_trial, shirt_color, pants_color, hair_style, hair_color,
			eyebrow_style, eye_style, mouth_style, last_location,
		})
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a CharListChar
	where       u8: Serialize<LE, W>,
	           u16: Serialize<LE, W>,
	           u32: Serialize<LE, W>,
	         ObjId: Serialize<LE, W>,
	      &'a [u8]: Serialize<LE, W>,
	  &'a LuWStr33: Serialize<LE, W>,
	    &'a ZoneId: Serialize<LE, W>,
	          bool: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.obj_id)?;
		writer.write(0u32)?; // unused
		writer.write(&self.char_name)?;
		writer.write(&self.pending_name)?;
		writer.write(self.requires_rename)?;
		writer.write(self.is_free_trial)?;
		writer.write(&[0; 10][..])?;

		writer.write(self.shirt_color)?;
		writer.write(&[0; 4][..])?;

		writer.write(self.pants_color)?;
		writer.write(self.hair_style)?;
		writer.write(self.hair_color)?;
		writer.write(&[0; 8][..])?;

		writer.write(self.eyebrow_style)?;
		writer.write(self.eye_style)?;
		writer.write(self.mouth_style)?;
		writer.write(&[0; 4][..])?;

		writer.write(&self.last_location)?;
		writer.write(&[0; 8][..])?;

		writer.write(0u16)?;
		Ok(())
	}
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum CharacterCreateResponse {
	Success,
	GeneralFailure,
	NameNotAllowed,
	PredefinedNameInUse,
	CustomNameInUse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterDeleteResponse {
	pub success: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferToWorld {
	pub redirect_ip: LuStr33,
	pub redirect_port: u16,
	pub is_maintenance_transfer: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlueprintLoadItemResponse {
	pub success: bool,
	pub item_id: ObjId,
	pub dest_item_id: ObjId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FriendRequest {
	pub sender_name: LuWStr33,
	pub is_best_friend_request: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamInvite {
	pub sender_name: LuWStr33,
	pub sender_id: ObjId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MinimumChatModeResponse {
	pub chat_mode: u8, // todo: type?
	pub chat_channel: u8, // todo: type?
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MinimumChatModeResponsePrivate {
	pub chat_mode: u8, // todo: type?
	pub chat_channel: u8, // todo: type?
	pub recipient_name: LuWStr33,
	pub recipient_gm_level: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFreeTrialStatus {
	pub is_free_trial: bool,
}
