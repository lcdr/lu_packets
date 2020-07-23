use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::VariantTests;

use crate::common::ServiceId;

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding=1]
#[repr(u32)]
pub enum GeneralMessage {
	Handshake(Handshake)
}

#[derive(Debug, PartialEq)]
pub struct Handshake {
	pub network_version: u32,
	pub service_id: ServiceId,
	pub process_id: u32,
	pub port: u16,
}

impl<R: Read+LERead> Deserialize<LE, R> for Handshake
	where    u8: Deserialize<LE, R>,
	        u16: Deserialize<LE, R>,
	  ServiceId: Deserialize<LE, R>,
	        u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let network_version = LERead::read(reader)?;
		let _: u32          = LERead::read(reader)?;
		let service_id      = LERead::read(reader)?;
		let _: u16          = LERead::read(reader)?;
		let process_id: u32 = LERead::read(reader)?;
		let port: u16       = LERead::read(reader)?;
		let mut unused = [0; 33];
		Read::read_exact(reader, &mut unused)?;
		Ok(Self { network_version, service_id, process_id, port })
	}
}

impl<'a, W: Write+LEWrite> Serialize<LE, W> for &'a Handshake
	where       u8: Serialize<LE, W>,
	            u16: Serialize<LE, W>,
	  &'a ServiceId: Serialize<LE, W>,
	            u32: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()> {
		LEWrite::write(writer, self.network_version)?;
		LEWrite::write(writer, 0u32)?;
		LEWrite::write(writer, &self.service_id)?;
		LEWrite::write(writer, 0u16)?;
		LEWrite::write(writer, &self.process_id)?;
		LEWrite::write(writer, &self.port)?;
		std::io::Write::write_all(writer, &[0; 33])
	}
}
