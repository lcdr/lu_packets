use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::{ServiceMessageD, ServiceMessageS};

use crate::common::{LuVarWStr, LuWStr33, ObjId};

#[derive(Debug, ServiceMessageD, ServiceMessageS)]
#[disc_padding=9]
#[repr(u32)]
pub enum ChatMessage {
	GeneralChatMessage(GeneralChatMessage) = 1,
	PrivateChatMessage(PrivateChatMessage) = 2,
	AddFriendResponse(AddFriendResponse) = 8,
	GetFriendsList = 10,
	GetIgnoreList = 13,
	TeamInviteResponse(TeamInviteResponse) = 16,
	TeamLeave(TeamLeave) = 18,
	TeamGetStatus = 21,
	RequestMinimumChatMode(RequestMinimumChatMode) = 50,
	RequestMinimumChatModePrivate(RequestMinimumChatModePrivate) = 51,
}

#[derive(Debug)]
pub struct GeneralChatMessage {
	pub chat_channel: u8, // todo: type?
	pub sender_name: LuWStr33,
	pub sender: ObjId,
	pub source_id: u16,
	pub sender_gm_level: u8,
	pub message: LuVarWStr<u32>,
}

impl<R: Read+LERead> Deserialize<LE, R> for GeneralChatMessage
	where   u8: Deserialize<LE, R>,
	       u16: Deserialize<LE, R>,
	       u32: Deserialize<LE, R>,
	  LuWStr33: Deserialize<LE, R>,
	     ObjId: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let chat_channel       = LERead::read(reader)?;
		let str_len: u32       = LERead::read(reader)?;
		let sender_name        = LERead::read(reader)?;
		let sender             = LERead::read(reader)?;
		let source_id          = LERead::read(reader)?;
		let sender_gm_level    = LERead::read(reader)?;
		let message = LuVarWStr::deser_content(reader, str_len)?;

		Ok(Self { chat_channel, sender_name, sender, source_id, sender_gm_level, message })
	}
}

impl<'a, W: Write+LEWrite> Serialize<LE, W> for &'a GeneralChatMessage
	where       u8: Serialize<LE, W>,
	           u16: Serialize<LE, W>,
	           u32: Serialize<LE, W>,
	  &'a LuWStr33: Serialize<LE, W>,
	         ObjId: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()> {
		LEWrite::write(writer, self.chat_channel)?;
		self.message.ser_len(writer)?;
		LEWrite::write(writer, &self.sender_name)?;
		LEWrite::write(writer, self.sender)?;
		LEWrite::write(writer, self.source_id)?;
		LEWrite::write(writer, self.sender_gm_level)?;
		self.message.ser_content(writer)
	}
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum PrivateChatMessageResponseCode {
	Sent,
	NotOnline,
	GeneralError,
	ReceivedNewWhisper,
	NotFriends,
	SenderFreeTrial,
	ReceiverFreeTrial,
}

#[derive(Debug)]
pub struct PrivateChatMessage {
	pub chat_channel: u8, // todo: type?
	pub sender_name: LuWStr33,
	pub sender: ObjId,
	pub source_id: u16,
	pub sender_gm_level: u8,
	pub recipient_name: LuWStr33,
	pub recipient_gm_level: u8,
	pub response_code: PrivateChatMessageResponseCode,
	pub message: LuVarWStr<u32>,
}

impl<R: Read+LERead> Deserialize<LE, R> for PrivateChatMessage
	where   u8: Deserialize<LE, R>,
	       u16: Deserialize<LE, R>,
	       u32: Deserialize<LE, R>,
	  LuWStr33: Deserialize<LE, R>,
	     ObjId: Deserialize<LE, R>,
	  PrivateChatMessageResponseCode: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let chat_channel       = LERead::read(reader)?;
		let str_len: u32       = LERead::read(reader)?;
		let sender_name        = LERead::read(reader)?;
		let sender             = LERead::read(reader)?;
		let source_id          = LERead::read(reader)?;
		let sender_gm_level    = LERead::read(reader)?;
		let recipient_name     = LERead::read(reader)?;
		let recipient_gm_level = LERead::read(reader)?;
		let response_code      = LERead::read(reader)?;
		let message = LuVarWStr::deser_content(reader, str_len)?;

		Ok(Self { chat_channel, sender_name, sender, source_id, sender_gm_level, recipient_name, recipient_gm_level, response_code, message })
	}
}
impl<'a, W: Write+LEWrite> Serialize<LE, W> for &'a PrivateChatMessage
	where       u8: Serialize<LE, W>,
	           u16: Serialize<LE, W>,
	           u32: Serialize<LE, W>,
	  &'a LuWStr33: Serialize<LE, W>,
	         ObjId: Serialize<LE, W>,
	  &'a PrivateChatMessageResponseCode: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()> {
		LEWrite::write(writer, self.chat_channel)?;
		self.message.ser_len(writer)?;
		LEWrite::write(writer, &self.sender_name)?;
		LEWrite::write(writer, self.sender)?;
		LEWrite::write(writer, self.source_id)?;
		LEWrite::write(writer, self.sender_gm_level)?;
		LEWrite::write(writer, &self.recipient_name)?;
		LEWrite::write(writer, self.recipient_gm_level)?;
		LEWrite::write(writer, &self.response_code)?;
		self.message.ser_content(writer)
	}
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum AddFriendResponseCode {
	Accepted,
	Rejected,
	Busy,
	Cancelled,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddFriendResponse {
	pub response_code: AddFriendResponseCode,
	pub friend_name: LuWStr33,
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum TeamInviteResponseCode {
	Accepted,
	Rejected,
	GeneralError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamInviteResponse {
	pub response_code: TeamInviteResponseCode,
	pub sender: ObjId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamLeave {
	pub unused: LuWStr33,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestMinimumChatMode {
	pub chat_channel: u8, // todo: separate type?
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestMinimumChatModePrivate {
	pub chat_channel: u8, // todo: separate type?
	pub recipient_name: LuWStr33,
}
