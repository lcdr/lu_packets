use std::io::Result as Res;
use std::net::Ipv4Addr;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

pub enum MessageId {
	ConnectedPong = 3,
	ConnectionRequestAccepted = 14,
	DisconnectionNotification = 19,
	UserMessage = 83,
}

macro_rules! rak_client_msg {
 ($T:ty) => {

#[derive(Debug)]
#[non_exhaustive]
pub enum Message {
	ConnectedPong($crate::raknet::client::ConnectedPong),
	ConnectionRequestAccepted($crate::raknet::client::ConnectionRequestAccepted),
	DisconnectionNotification,
	UserMessage($T),
}

impl<W: LEWrite> endio::Serialize<LE, W> for &Message
	where         u8: endio::Serialize<LE, W>,
		for<'a> &'a $crate::raknet::client::ConnectionRequestAccepted: endio::Serialize<LE, W>,
		for<'b> &'b $crate::raknet::client::ConnectedPong: endio::Serialize<LE, W>,
		for<'c> &'c $T: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			Message::ConnectedPong(msg) => {
				writer.write($crate::raknet::client::MessageId::ConnectedPong as u8)?;
				writer.write(msg)?;
			}
			Message::ConnectionRequestAccepted(msg) => {
				writer.write($crate::raknet::client::MessageId::ConnectionRequestAccepted as u8)?;
				writer.write(msg)?;
			}
			Message::DisconnectionNotification => {
				writer.write($crate::raknet::client::MessageId::DisconnectionNotification as u8)?;
			}
			Message::UserMessage(msg) => {
				writer.write($crate::raknet::client::MessageId::UserMessage as u8)?;
				writer.write(msg)?;
			}
		}
		Ok(())
	}
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

#[derive(Debug)]
pub struct ConnectedPong {
	pub ping_send_time: u32,
}


impl<'a, W: LEWrite> Serialize<LE, W> for &'a ConnectedPong
	where u32: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.ping_send_time)
	}
}

#[derive(Debug)]
pub struct ConnectionRequestAccepted {
	pub peer_ip: Ipv4Addr,
	pub peer_port: u16,
	pub local_ip: Ipv4Addr,
	pub local_port: u16,
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a ConnectionRequestAccepted
	where          u16: Serialize<LE, W>,
	               u32: Serialize<LE, W>,
		for<'b> &'b [u8]: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(&self.peer_ip.octets()[..])?;
		writer.write(self.peer_port)?;
		writer.write(&[0u8; 2][..])?;
		writer.write(&self.local_ip.octets()[..])?;
		writer.write(self.local_port)
	}
}
