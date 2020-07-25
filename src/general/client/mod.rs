use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;

use crate::common::ServiceId;

pub type Message<C> = crate::raknet::client::Message<LuMessage<C>>;

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[test_params(crate::world::client::ClientMessage)]
#[repr(u16)]
pub enum LuMessage<C> {
	General(GeneralMessage) = ServiceId::General as u16,
	Client(C) = ServiceId::Client as u16,
}

impl<C> From<LuMessage<C>> for Message<C> {
	fn from(msg: LuMessage<C>) -> Self {
		Message::UserMessage(msg)
	}
}

impl<C> From<GeneralMessage> for Message<C> {
	fn from(msg: GeneralMessage) -> Self {
		LuMessage::General(msg).into()
	}
}

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding=1]
#[repr(u32)]
pub enum GeneralMessage {
	Handshake(Handshake),
	DisconnectNotify(DisconnectNotify),
}

impl<C> From<Handshake> for Message<C> {
	fn from(msg: Handshake) -> Self {
		GeneralMessage::Handshake(msg).into()
	}
}

impl<C> From<DisconnectNotify> for Message<C> {
	fn from(msg: DisconnectNotify) -> Self {
		GeneralMessage::DisconnectNotify(msg).into()
	}
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding=41]
pub struct Handshake {
	pub network_version: u32,
	#[padding=4]
	pub service_id: ServiceId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum DisconnectNotify {
	UnknownServerError,
	WrongGameVersion(u32),
	WrongServerVersion(u32),
	ConnectionOnInvalidPort,
	DuplicateLogin,
	ServerShutdown,
	UnableToLoadMap,
	InvalidSessionKey,
	AccountNotInPendingList,
	CharacterNotFound,
	CharacterCorruption,
	Kick,
	SaveFailure,
	FreeTrialExpired,
	PlayScheduleTimeUp,
}
