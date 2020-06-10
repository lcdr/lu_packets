use std::io::Result as Res;
use std::net::Ipv4Addr;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

// todo[chalk]: generic type instead of macro
macro_rules! rak_client_msg {
 ($T:ty) => {
#[derive(Debug, Serialize)]
#[non_exhaustive]
#[repr(u8)]
pub enum Message {
	ConnectedPong($crate::raknet::client::ConnectedPong) = 3,
	ConnectionRequestAccepted($crate::raknet::client::ConnectionRequestAccepted) = 14,
	DisconnectionNotification = 19,
	UserMessage($T) = 83,
}

impl From<$crate::raknet::client::ConnectedPong> for Message {
	fn from(msg: $crate::raknet::client::ConnectedPong) -> Self {
		Message::ConnectedPong(msg)
	}
}

impl From<$crate::raknet::client::ConnectionRequestAccepted> for Message {
	fn from(msg: $crate::raknet::client::ConnectionRequestAccepted) -> Self {
		Message::ConnectionRequestAccepted(msg)
	}
}
}
}

#[derive(Debug, Serialize)]
pub struct ConnectedPong {
	pub ping_send_time: u32,
}

#[derive(Debug)]
pub struct ConnectionRequestAccepted {
	pub peer_ip: Ipv4Addr,
	pub peer_port: u16,
	pub local_ip: Ipv4Addr,
	pub local_port: u16,
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a ConnectionRequestAccepted
	where       u16: Serialize<LE, W>,
	            u32: Serialize<LE, W>,
	   &'a Ipv4Addr: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(&self.peer_ip)?;
		writer.write(self.peer_port)?;
		writer.write(0u16)?;
		writer.write(&self.local_ip)?;
		writer.write(self.local_port)
	}
}
