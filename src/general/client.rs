use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::ServiceMessageS;

use crate::common::ServiceId;

pub type Message<C> = crate::raknet::client::Message<LuMessage<C>>;

#[derive(Debug, Serialize)]
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

#[derive(Debug, ServiceMessageS)]
#[non_exhaustive]
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

#[derive(Debug)]
pub struct Handshake {
	pub network_version: u32,
	pub service_id: ServiceId,
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a Handshake
	where       u16: Serialize<LE, W>,
	            u32: Serialize<LE, W>,
	  &'a ServiceId: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.network_version)?;
		writer.write(0u32)?;
		writer.write(&self.service_id)?;
		writer.write(0u16)?;
		Ok(())
	}
}

#[derive(Debug, Serialize)]
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
