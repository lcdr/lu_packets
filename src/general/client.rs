use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

use crate::common::ServiceId;

#[derive(Debug)]
#[non_exhaustive]
#[repr(u32)]
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
		let disc = unsafe { *(self as *const GeneralMessage as *const u32) };
		writer.write(disc)?;
		writer.write(0u8)?;
		match self {
			GeneralMessage::Handshake(msg) => {
				writer.write(msg)?;
			}
			GeneralMessage::DisconnectNotify(msg) => {
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
