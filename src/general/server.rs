use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;

use crate::common::{err, ServiceId};

enum GeneralId {
	Handshake = 0,
}

#[derive(Debug)]
pub enum GeneralMessage {
	Handshake(Handshake)
}

impl<R: LERead> Deserialize<LE, R> for GeneralMessage
	where    u8: Deserialize<LE, R>,
	        u32: Deserialize<LE, R>,
	  Handshake: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let packet_id: u32 = reader.read()?;
		let _padding: u8   = reader.read()?;
		if packet_id == GeneralId::Handshake as u32 {
			Ok(GeneralMessage::Handshake(reader.read()?))
		} else {
			err("invalid general id")
		}
	}
}

#[derive(Debug)]
pub struct Handshake {
	pub network_version: u32,
	pub service_id: ServiceId,
}

impl<R: LERead> Deserialize<LE, R> for Handshake
	where u8: Deserialize<LE, R>,
	     u16: Deserialize<LE, R>,
	     u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let network_version = reader.read()?;
		let _: u32          = reader.read()?;
		let service_id      = reader.read()?;
		let _: u16          = reader.read()?;
		Ok(Self { network_version, service_id })
	}
}
