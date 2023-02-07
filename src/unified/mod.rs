use crate::auth::client::LoginResponse;
use crate::auth::server::AuthMessage;
use crate::chat::client::AchievementNotify;
use crate::chat::server::{
	AddFriendRequest as ChatAddFriendRequest, AddFriendResponse as ChatAddFriendResponse,
	AddIgnore, RequestMinimumChatMode, RequestMinimumChatModePrivate, TeamInvite as ChatTeamInvite,
	TeamInviteResponse, TeamLeave,
};
use crate::chat::{GeneralChatMessage, PrivateChatMessage};
use crate::common::ServiceId;
use crate::general::client::GeneralMessage;
use crate::raknet::client::{
	replica::{ReplicaConstruction, ReplicaSerialization},
	ConnectedPong, ConnectionRequestAccepted,
};
use crate::raknet::server::{ConnectionRequest, InternalPing, NewIncomingConnection};
use crate::world::client::{
	AddFriendRequest, AddFriendResponse, BlueprintLoadItemResponse, BlueprintSaveResponse,
	CharacterCreateResponse, CharacterDeleteResponse, CharacterListResponse, ChatModerationString,
	CreateCharacter, FriendUpdateNotify, GetFriendsListResponse, GetIgnoreListResponse,
	LoadStaticZone, MinimumChatModeResponse, MinimumChatModeResponsePrivate, TeamInvite,
	TransferToWorld, UpdateFreeTrialStatus,
};
use crate::world::gm::client::SubjectGameMessage;
use crate::world::server::WorldMessage;
use endio::{Deserialize, Serialize};
use lu_packets_derive::MessageFromVariants;

#[derive(Debug, Deserialize, Serialize, PartialEq, MessageFromVariants)]
#[non_exhaustive]
#[repr(u8)]
pub enum Message {
	InternalPing(InternalPing) = 0,
	ConnectedPong(ConnectedPong) = 3,
	ConnectionRequest(ConnectionRequest) = 4,
	ConnectionRequestAccepted(ConnectionRequestAccepted) = 14,
	NewIncomingConnection(NewIncomingConnection) = 17,
	DisconnectionNotification = 19,
	ReplicaConstruction(ReplicaConstruction) = 36,
	ReplicaSerialization(ReplicaSerialization) = 39,
	UserMessage(UserMessage) = 83,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, MessageFromVariants)]
#[repr(u16)]
pub enum UserMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	Client(AnyClientMessage) = ServiceId::Client as u16,
	Chat(AnyChatMessage) = ServiceId::Chat as u16,
	World(WorldMessage) = ServiceId::World as u16,
	Auth(AuthMessage) = ServiceId::Auth as u16,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, MessageFromVariants)]
#[non_exhaustive]
#[post_disc_padding = 1]
#[repr(u32)]
pub enum AnyClientMessage {
	LoginResponse(LoginResponse) = 0,
	LoadStaticZone(LoadStaticZone) = 2,
	CreateCharacter(CreateCharacter) = 4,
	CharacterListResponse(CharacterListResponse) = 6,
	CharacterCreateResponse(CharacterCreateResponse) = 7,
	CharacterDeleteResponse(CharacterDeleteResponse) = 11,
	SubjectGameMessage(SubjectGameMessage) = 12,
	TransferToWorld(TransferToWorld) = 14,
	BlueprintSaveResponse(BlueprintSaveResponse) = 21,
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

#[derive(Debug, Deserialize, Serialize, PartialEq, MessageFromVariants)]
#[non_exhaustive]
#[post_disc_padding = 9]
#[repr(u32)]
pub enum AnyChatMessage {
	GeneralChatMessage(GeneralChatMessage) = 1,
	PrivateChatMessage(PrivateChatMessage) = 2,
	AddFriendRequest(ChatAddFriendRequest) = 7,
	AddFriendResponse(ChatAddFriendResponse) = 8,
	GetFriendsList = 10,
	AddIgnore(AddIgnore) = 11,
	GetIgnoreList = 13,
	TeamInvite(ChatTeamInvite) = 15,
	TeamInviteResponse(TeamInviteResponse) = 16,
	TeamLeave(TeamLeave) = 18,
	TeamGetStatus = 21,
	RequestMinimumChatMode(RequestMinimumChatMode) = 50,
	RequestMinimumChatModePrivate(RequestMinimumChatModePrivate) = 51,
	AchievementNotify(AchievementNotify) = 59,
}
