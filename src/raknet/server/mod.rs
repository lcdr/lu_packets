//! Server-received raknet messages.
use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::VariantTests;

use super::SystemAddress;

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[test_params(crate::world::server::LuMessage)]
#[non_exhaustive]
#[repr(u8)]
pub enum Message<U> {
	InternalPing(InternalPing) = 0,
	ConnectionRequest(ConnectionRequest) = 4,
	NewIncomingConnection(NewIncomingConnection) = 17,
	DisconnectionNotification = 19,
	UserMessage(U) = 83,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct InternalPing {
	pub send_time: u32,
}

#[derive(Debug, PartialEq)]
pub struct ConnectionRequest {
	pub password: Box<[u8]>,
}

impl<R: Read> Deserialize<LE, R> for ConnectionRequest {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let mut password = vec![];
		reader.read_to_end(&mut password)?;
		let password = password.into_boxed_slice();
		Ok(Self { password })
	}
}

impl<'a, W: Write> Serialize<LE, W> for &'a ConnectionRequest {
	fn serialize(self, writer: &mut W) -> Res<()> {
		writer.write_all(&self.password)?;
		Ok(())
	}
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct NewIncomingConnection {
	pub peer_addr: SystemAddress,
	pub local_addr: SystemAddress,
}
