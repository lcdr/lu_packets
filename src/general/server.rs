use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use endio::LittleEndian as LE;

use crate::common::ServiceId;

#[derive(Debug, Deserialize, Serialize)]
#[post_disc_padding=1]
#[repr(u32)]
pub enum GeneralMessage {
	Handshake(Handshake)
}

#[derive(Debug)]
pub struct Handshake {
	pub network_version: u32,
	pub service_id: ServiceId,
}

impl<R: LERead> Deserialize<LE, R> for Handshake
	where    u8: Deserialize<LE, R>,
	        u16: Deserialize<LE, R>,
	  ServiceId: Deserialize<LE, R>,
	        u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let network_version = reader.read()?;
		let _: u32          = reader.read()?;
		let service_id      = reader.read()?;
		let _: u16          = reader.read()?;
		Ok(Self { network_version, service_id })
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a Handshake
	where       u8: Serialize<LE, W>,
	            u16: Serialize<LE, W>,
	  &'a ServiceId: Serialize<LE, W>,
	            u32: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()> {
		writer.write(self.network_version)?;
		writer.write(0u32)?;
		writer.write(&self.service_id)?;
		writer.write(0u16)
	}
}
