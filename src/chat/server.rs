use std::io::Read;
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;
use lu_packets_derive::ServiceMessageD;

use crate::common::{LuWStr33, ObjId};

#[derive(Debug, ServiceMessageD)]
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
	pub message: String,
}

impl<R: Read+LERead> Deserialize<LE, R> for GeneralChatMessage
	where   u8: Deserialize<LE, R>,
	       u16: Deserialize<LE, R>,
	       u32: Deserialize<LE, R>,
	  LuWStr33: Deserialize<LE, R>,
	     ObjId: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let chat_channel       = LERead::read(reader)?;
		let string_len: u32    = LERead::read(reader)?;
		let sender_name        = LERead::read(reader)?;
		let sender             = LERead::read(reader)?;
		let source_id          = LERead::read(reader)?;
		let sender_gm_level    = LERead::read(reader)?;
		let mut string = vec![0; (string_len*2) as usize];
		let mut taken = Read::take(reader, (string_len*2) as u64);
		Read::read(&mut taken, &mut string)?;
		let string_slice: &[u16] = unsafe { std::slice::from_raw_parts(string.as_ptr() as *const u16, string_len as usize - 1) };
		let message = String::from_utf16_lossy(string_slice);

		Ok(Self { chat_channel, sender_name, sender, source_id, sender_gm_level, message })
	}
}
#[derive(Debug, Deserialize)]
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
	pub message: String,
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
		let string_len: u32    = LERead::read(reader)?;
		let sender_name        = LERead::read(reader)?;
		let sender             = LERead::read(reader)?;
		let source_id          = LERead::read(reader)?;
		let sender_gm_level    = LERead::read(reader)?;
		let recipient_name     = LERead::read(reader)?;
		let recipient_gm_level = LERead::read(reader)?;
		let response_code      = LERead::read(reader)?;
		let mut string = vec![0; (string_len*2) as usize];
		let mut taken = Read::take(reader, (string_len*2) as u64);
		Read::read(&mut taken, &mut string)?;
		let string_slice: &[u16] = unsafe { std::slice::from_raw_parts(string.as_ptr() as *const u16, string_len as usize - 1) };
		let message = String::from_utf16_lossy(string_slice);

		Ok(Self { chat_channel, sender_name, sender, source_id, sender_gm_level, recipient_name, recipient_gm_level, response_code, message })
	}
}

#[derive(Debug, Deserialize)]
#[repr(u8)]
pub enum AddFriendResponseCode {
	Accepted,
	Rejected,
	Busy,
	Cancelled,
}

#[derive(Debug, Deserialize)]
pub struct AddFriendResponse {
	pub response_code: AddFriendResponseCode,
	pub friend_name: LuWStr33,
}

#[derive(Debug, Deserialize)]
#[repr(u8)]
pub enum TeamInviteResponseCode {
	Accepted,
	Rejected,
	GeneralError,
}

#[derive(Debug, Deserialize)]
pub struct TeamInviteResponse {
	pub response_code: TeamInviteResponseCode,
	pub sender: ObjId,
}

#[derive(Debug, Deserialize)]
pub struct TeamLeave {
	pub unused: LuWStr33,
}

#[derive(Debug, Deserialize)]
pub struct RequestMinimumChatMode {
	pub chat_channel: u8, // todo: separate type?
}

#[derive(Debug, Deserialize)]
pub struct RequestMinimumChatModePrivate {
	pub chat_channel: u8, // todo: separate type?
	pub recipient_name: LuWStr33,
}
