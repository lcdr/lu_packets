use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

use crate::common::{ServiceId};

enum GeneralId {
	Handshake = 0,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum GeneralMessage {
	Handshake(Handshake)
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a GeneralMessage
	where u8: Serialize<LE, W>,
	     u32: Serialize<LE, W>,
	     &'a Handshake: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			GeneralMessage::Handshake(handshake) => {
				writer.write(GeneralId::Handshake as u32)?;
				writer.write(0u8)?;
				writer.write(handshake)?;
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
