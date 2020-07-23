use std::net::Ipv4Addr;

use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[test_params(crate::world::client::LuMessage)]
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ConnectedPong {
	pub ping_send_time: u32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ConnectionRequestAccepted {
	pub peer_ip: Ipv4Addr,
	pub peer_port: u16,
	#[padding=2]
	pub local_ip: Ipv4Addr,
	pub local_port: u16,
}

