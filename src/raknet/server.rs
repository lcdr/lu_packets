//! All packets a world server can receive.
use std::io::Read;
use std::io::Result as Res;

use endio::Deserialize;
use endio::LittleEndian as LE;

use crate::common::SystemAddress;

// todo[chalk]: generic type instead of macro
macro_rules! rak_server_msg {
 ($T:ty) => {
#[derive(Debug, Deserialize)]
#[non_exhaustive]
#[repr(u8)]
pub enum Message {
	InternalPing($crate::raknet::server::InternalPing) = 0,
	ConnectionRequest($crate::raknet::server::ConnectionRequest) = 4,
	NewIncomingConnection($crate::raknet::server::NewIncomingConnection) = 17,
	DisconnectionNotification = 19,
	UserMessage($T) = 83,
}
}
}

#[derive(Debug, Deserialize)]
pub struct InternalPing {
	pub send_time: u32
}

#[derive(Debug)]
pub struct ConnectionRequest {
	pub password: Box<[u8]>
}

impl<R: Read> Deserialize<LE, R> for ConnectionRequest {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let mut password = vec![];
		Read::read_to_end(reader, &mut password)?;
		let password = password.into_boxed_slice();
		Ok(Self { password })
	}
}

#[derive(Debug, Deserialize)]
pub struct NewIncomingConnection {
	peer_addr: SystemAddress,
	local_addr: SystemAddress,
}
