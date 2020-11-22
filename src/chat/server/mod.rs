use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;

use crate::common::{LuWString33, ObjId};
use super::{ChatChannel, GeneralChatMessage, PrivateChatMessage};

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding=9]
#[repr(u32)]
pub enum ChatMessage {
	GeneralChatMessage(GeneralChatMessage) = 1,
	PrivateChatMessage(PrivateChatMessage) = 2,
	AddFriendRequest(AddFriendRequest) = 7,
	AddFriendResponse(AddFriendResponse) = 8,
	GetFriendsList = 10,
	AddIgnore(AddIgnore) = 11,
	GetIgnoreList = 13,
	TeamInviteResponse(TeamInviteResponse) = 16,
	TeamLeave(TeamLeave) = 18,
	TeamGetStatus = 21,
	RequestMinimumChatMode(RequestMinimumChatMode) = 50,
	RequestMinimumChatModePrivate(RequestMinimumChatModePrivate) = 51,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum AddFriendResponseCode {
	Accepted,
	Rejected,
	Busy,
	Cancelled,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AddFriendRequest {
	pub friend_name: LuWString33,
	pub is_best_friend: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AddFriendResponse {
	pub response_code: AddFriendResponseCode,
	pub friend_name: LuWString33,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AddIgnore {
	pub char_name: LuWString33,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum TeamInviteResponseCode {
	Accepted,
	Rejected,
	GeneralError,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TeamInviteResponse {
	pub response_code: TeamInviteResponseCode,
	pub sender: ObjId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TeamLeave {
	pub unused: LuWString33,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct RequestMinimumChatMode {
	pub chat_channel: ChatChannel,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct RequestMinimumChatModePrivate {
	pub chat_channel: ChatChannel,
	pub recipient_name: LuWString33,
}
