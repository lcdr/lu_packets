use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

use crate::common::{ServiceId};

enum GeneralId {
	Handshake = 0,
	DisconnectNotify = 1,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum GeneralMessage {
	Handshake(Handshake),
	DisconnectNotify(DisconnectNotify),
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a GeneralMessage
	where u8: Serialize<LE, W>,
	     u32: Serialize<LE, W>,
	     &'a Handshake: Serialize<LE, W>,
	     &'a DisconnectNotify: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			GeneralMessage::Handshake(msg) => {
				writer.write(GeneralId::Handshake as u32)?;
				writer.write(0u8)?;
				writer.write(msg)?;
			}
			GeneralMessage::DisconnectNotify(msg) => {
				writer.write(GeneralId::DisconnectNotify as u32)?;
				writer.write(0u8)?;
				writer.write(msg)?;
			}
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct Handshake {
	pub network_version: u32,
	pub service_id: ServiceId,
}

impl<W: LEWrite> Serialize<LE, W> for &Handshake
	where u8 : Serialize<LE, W>,
	     u16 : Serialize<LE, W>,
	     u32 : Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.network_version)?;
		writer.write(0u32)?;
		writer.write(self.service_id)?;
		writer.write(0u16)?;
		Ok(())
	}
}

#[derive(Debug)]
pub enum DisconnectReasonId {
	UnknownServerError = 0,
	WrongGameVersion = 1,
	WrongServerVersion = 2,
	ConnectionOnInvalidPort = 3,
	DuplicateLogin = 4,
	ServerShutdown = 5,
	UnableToLoadMap = 6,
	InvalidSessionKey = 7,
	AccountNotInPendingList = 8,
	CharacterNotFound = 9,
	CharacterCorruption = 10,
	Kick = 11,
	SaveFailure = 12,
	FreeTrialExpired = 13,
	PlayScheduleTimeUp = 14,
}

#[derive(Debug)]
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

impl<W: LEWrite> Serialize<LE, W> for &DisconnectNotify
	where u32 : Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			DisconnectNotify::UnknownServerError => {
				writer.write(DisconnectReasonId::UnknownServerError as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::WrongGameVersion(version) => {
				writer.write(DisconnectReasonId::WrongGameVersion as u32)?;
				writer.write(*version)?;
			}
			DisconnectNotify::WrongServerVersion(version) => {
				writer.write(DisconnectReasonId::WrongServerVersion as u32)?;
				writer.write(*version)?;
			}
			DisconnectNotify::ConnectionOnInvalidPort => {
				writer.write(DisconnectReasonId::ConnectionOnInvalidPort as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::DuplicateLogin => {
				writer.write(DisconnectReasonId::DuplicateLogin as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::ServerShutdown => {
				writer.write(DisconnectReasonId::ServerShutdown as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::UnableToLoadMap => {
				writer.write(DisconnectReasonId::UnableToLoadMap as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::InvalidSessionKey => {
				writer.write(DisconnectReasonId::InvalidSessionKey as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::AccountNotInPendingList => {
				writer.write(DisconnectReasonId::AccountNotInPendingList as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::CharacterNotFound => {
				writer.write(DisconnectReasonId::CharacterNotFound as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::CharacterCorruption => {
				writer.write(DisconnectReasonId::CharacterCorruption as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::Kick => {
				writer.write(DisconnectReasonId::Kick as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::SaveFailure => {
				writer.write(DisconnectReasonId::SaveFailure as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::FreeTrialExpired => {
				writer.write(DisconnectReasonId::FreeTrialExpired as u32)?;
				writer.write(0u32)?;
			}
			DisconnectNotify::PlayScheduleTimeUp => {
				writer.write(DisconnectReasonId::PlayScheduleTimeUp as u32)?;
				writer.write(0u32)?;
			}
		}
		Ok(())
	}
}
