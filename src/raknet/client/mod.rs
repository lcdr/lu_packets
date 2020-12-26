//! Client-received raknet messages.
pub mod replica;

use endio::{Deserialize, Serialize};
use lu_packets_derive::ReplicaVariantTests;

use super::SystemAddress;
use replica::ReplicaConstruction;

#[derive(Debug, Deserialize, PartialEq, Serialize, ReplicaVariantTests)]
#[test_params(crate::world::client::LuMessage)]
#[non_exhaustive]
#[repr(u8)]
pub enum Message<U> {
	ConnectedPong(ConnectedPong) = 3,
	ConnectionRequestAccepted(ConnectionRequestAccepted) = 14,
	DisconnectionNotification = 19,
	ReplicaConstruction(ReplicaConstruction) = 36,
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
impl<U> From<ReplicaConstruction> for Message<U> {
	fn from(msg: ReplicaConstruction) -> Self {
		Message::ReplicaConstruction(msg)
	}
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ConnectedPong {
	pub ping_send_time: u32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ConnectionRequestAccepted {
	pub peer_addr: SystemAddress,
	#[padding=2]
	pub local_addr: SystemAddress,
}
