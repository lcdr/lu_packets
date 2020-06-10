use std::io::Result as Res;
use std::net::Ipv4Addr;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[repr(u8)]
pub enum Message<U> {
	ConnectedPong(ConnectedPong) = 3,
	ConnectionRequestAccepted(ConnectionRequestAccepted) = 14,
	DisconnectionNotification = 19,
	UserMessage(U) = 83,
}

impl<U> From<ConnectedPong> for Message<U> {
	fn from(msg: ConnectedPong) -> Self {
		Message::ConnectedPong(msg)
	}
}

impl<U> From<ConnectionRequestAccepted> for Message<U> {
	fn from(msg: ConnectionRequestAccepted) -> Self {
		Message::ConnectionRequestAccepted(msg)
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
