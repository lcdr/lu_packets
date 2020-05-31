use std::io::Read;
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;

use crate::common::{err, LuWStr33, ObjId};

enum ChatId {
	GeneralChatMessage = 1,
	PrivateChatMessage = 2,
	AddFriendResponse = 8,
	GetFriendsList = 10,
	GetIgnoreList = 13,
	TeamInviteResponse = 16,
	TeamLeave = 18,
	TeamGetStatus = 21,
	RequestMinimumChatMode = 50,
	RequestMinimumChatModePrivate = 51,
}

#[derive(Debug)]
pub enum ChatMessage {
	GeneralChatMessage(GeneralChatMessage),
	PrivateChatMessage(PrivateChatMessage),
	AddFriendResponse(AddFriendResponse),
	GetFriendsList,
	GetIgnoreList,
	TeamInviteResponse(TeamInviteResponse),
	TeamLeave,
	TeamGetStatus,
	RequestMinimumChatMode(RequestMinimumChatMode),
	RequestMinimumChatModePrivate(RequestMinimumChatModePrivate),
}

impl<R: Read+LERead> Deserialize<LE, R> for ChatMessage
	where                        u8: Deserialize<LE, R>,
	                            u32: Deserialize<LE, R>,
	                            u64: Deserialize<LE, R>,
	             GeneralChatMessage: Deserialize<LE, R>,
	             PrivateChatMessage: Deserialize<LE, R>,
	              AddFriendResponse: Deserialize<LE, R>,
	             TeamInviteResponse: Deserialize<LE, R>,
	         RequestMinimumChatMode: Deserialize<LE, R>,
	  RequestMinimumChatModePrivate: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let packet_id: u32       = LERead::read(reader)?;
		let _padding: u8         = LERead::read(reader)?;
		let routed_obj_id: u64   = LERead::read(reader)?;
		assert_eq!(routed_obj_id, 0);
		Ok(if packet_id == ChatId::GeneralChatMessage as u32 {
			Self::GeneralChatMessage(LERead::read(reader)?)
		} else if packet_id == ChatId::PrivateChatMessage as u32 {
			Self::PrivateChatMessage(LERead::read(reader)?)
		} else if packet_id == ChatId::AddFriendResponse as u32 {
			Self::AddFriendResponse(LERead::read(reader)?)
		} else if packet_id == ChatId::GetFriendsList as u32 {
			Self::GetFriendsList
		} else if packet_id == ChatId::GetIgnoreList as u32 {
			Self::GetIgnoreList
		} else if packet_id == ChatId::TeamGetStatus as u32 {
			Self::TeamGetStatus
		} else if packet_id == ChatId::TeamInviteResponse as u32 {
			Self::TeamInviteResponse(LERead::read(reader)?)
		} else if packet_id == ChatId::TeamLeave as u32 {
			let mut unused = [0; 66];
			Read::read(reader, &mut unused)?;
			Self::TeamLeave
		} else if packet_id == ChatId::RequestMinimumChatMode as u32 {
			Self::RequestMinimumChatMode(LERead::read(reader)?)
		} else if packet_id == ChatId::RequestMinimumChatModePrivate as u32 {
			Self::RequestMinimumChatModePrivate(LERead::read(reader)?)
		} else {
			return err("chat id", packet_id);
		})
	}
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
#[derive(Debug)]
pub enum PrivateChatMessageResponseCode {
	Sent = 0,
	NotOnline = 1,
	GeneralError = 2,
	ReceivedNewWhisper = 3,
	NotFriends = 4,
	SenderFreeTrial = 5,
	ReceiverFreeTrial = 6,
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
	     ObjId: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let chat_channel       = LERead::read(reader)?;
		let string_len: u32    = LERead::read(reader)?;
		let sender_name        = LERead::read(reader)?;
		let sender             = LERead::read(reader)?;
		let source_id          = LERead::read(reader)?;
		let sender_gm_level    = LERead::read(reader)?;
		let recipient_name     = LERead::read(reader)?;
		let recipient_gm_level = LERead::read(reader)?;
		let response_code: u8  = LERead::read(reader)?;
		let response_code = if response_code == PrivateChatMessageResponseCode::Sent as u8 {
			PrivateChatMessageResponseCode::Sent
		} else if response_code == PrivateChatMessageResponseCode::NotOnline as u8 {
			PrivateChatMessageResponseCode::NotOnline
		} else if response_code == PrivateChatMessageResponseCode::GeneralError as u8 {
			PrivateChatMessageResponseCode::GeneralError
		} else if response_code == PrivateChatMessageResponseCode::ReceivedNewWhisper as u8 {
			PrivateChatMessageResponseCode::ReceivedNewWhisper
		} else if response_code == PrivateChatMessageResponseCode::NotFriends as u8 {
			PrivateChatMessageResponseCode::NotFriends
		} else if response_code == PrivateChatMessageResponseCode::SenderFreeTrial as u8 {
			PrivateChatMessageResponseCode::SenderFreeTrial
		} else if response_code == PrivateChatMessageResponseCode::ReceiverFreeTrial as u8 {
			PrivateChatMessageResponseCode::ReceiverFreeTrial
		} else {
			return err("private chat message response code", response_code);
		};
		let mut string = vec![0; (string_len*2) as usize];
		let mut taken = Read::take(reader, (string_len*2) as u64);
		Read::read(&mut taken, &mut string)?;
		let string_slice: &[u16] = unsafe { std::slice::from_raw_parts(string.as_ptr() as *const u16, string_len as usize - 1) };
		let message = String::from_utf16_lossy(string_slice);

		Ok(Self { chat_channel, sender_name, sender, source_id, sender_gm_level, recipient_name, recipient_gm_level, response_code, message })
	}
}

#[derive(Debug)]
pub enum AddFriendResponseCode {
	Accepted = 0,
	Rejected = 1,
	Busy = 2,
	Cancelled = 3,
}

#[derive(Debug)]
pub struct AddFriendResponse {
	pub response_code: AddFriendResponseCode,
	pub friend_name: LuWStr33,
}

impl<R: LERead> Deserialize<LE, R> for AddFriendResponse
	where   u8: Deserialize<LE, R>,
	  LuWStr33: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let response_code: u8 = reader.read()?;
		let response_code = if response_code == AddFriendResponseCode::Accepted as u8 {
			AddFriendResponseCode::Accepted
		} else if response_code == AddFriendResponseCode::Rejected as u8 {
			AddFriendResponseCode::Rejected
		} else if response_code == AddFriendResponseCode::Busy as u8 {
			AddFriendResponseCode::Busy
		} else if response_code == AddFriendResponseCode::Cancelled as u8 {
			AddFriendResponseCode::Cancelled
		} else {
			return err("add friend response code", response_code);
		};
		let friend_name = reader.read()?;

		Ok(Self { response_code, friend_name })
	}
}

#[derive(Debug)]
pub enum TeamInviteResponseCode {
	Accepted = 0,
	Rejected = 1,
	GeneralError = 2,
}

#[derive(Debug)]
pub struct TeamInviteResponse {
	pub response_code: TeamInviteResponseCode,
	pub sender: ObjId,
}

impl<R: LERead> Deserialize<LE, R> for TeamInviteResponse
	where u8: Deserialize<LE, R>,
	   ObjId: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let response_code: u8 = reader.read()?;
		let response_code = if response_code == TeamInviteResponseCode::Accepted as u8 {
			TeamInviteResponseCode::Accepted
		} else if response_code == TeamInviteResponseCode::Rejected as u8 {
			TeamInviteResponseCode::Rejected
		} else if response_code == TeamInviteResponseCode::GeneralError as u8 {
			TeamInviteResponseCode::GeneralError
		} else {
			return err("team invite response code", response_code);
		};
		let sender = reader.read()?;
		Ok(Self { response_code, sender })
	}
}

#[derive(Debug)]
pub struct RequestMinimumChatMode {
	pub chat_channel: u8, // todo: separate type?
}

impl<R: LERead> Deserialize<LE, R> for RequestMinimumChatMode
	where u8: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let chat_channel = reader.read()?;
		Ok(Self { chat_channel })
	}
}

#[derive(Debug)]
pub struct RequestMinimumChatModePrivate {
	pub chat_channel: u8, // todo: separate type?
	pub recipient_name: LuWStr33,
}

impl<R: LERead> Deserialize<LE, R> for RequestMinimumChatModePrivate
	where   u8: Deserialize<LE, R>,
	  LuWStr33: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let chat_channel   = reader.read()?;
		let recipient_name = reader.read()?;
		Ok(Self { chat_channel, recipient_name })
	}
}
