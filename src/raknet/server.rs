//! All packets a world server can receive.
use std::io::Read;
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;

use crate::common::{SystemAddress};

pub enum MessageId {
	InternalPing = 0,
	ConnectionRequest = 4,
	NewIncomingConnection = 17,
	UserMessage = 83,
}

macro_rules! rak_server_msg {
 ($T:ty) => {

#[derive(Debug)]
#[non_exhaustive]
pub enum Message {
	InternalPing($crate::raknet::server::InternalPing),
	ConnectionRequest($crate::raknet::server::ConnectionRequest),
	NewIncomingConnection($crate::raknet::server::NewIncomingConnection),
	UserMessage($T),
}

impl<R: endio::LERead> endio::Deserialize<LE, R> for Message
	where                u8: endio::Deserialize<LE, R>,
	           $crate::raknet::server::InternalPing: endio::Deserialize<LE, R>,
	      $crate::raknet::server::ConnectionRequest: endio::Deserialize<LE, R>,
	  $crate::raknet::server::NewIncomingConnection: endio::Deserialize<LE, R>,
	              $T: endio::Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let message_id: u8 = LERead::read(reader)?;
		Ok(if message_id == $crate::raknet::server::MessageId::InternalPing as u8 {
			Self::InternalPing(LERead::read(reader)?)
		}	else if message_id == $crate::raknet::server::MessageId::ConnectionRequest as u8 {
			Self::ConnectionRequest(LERead::read(reader)?)
		}	else if message_id == $crate::raknet::server::MessageId::NewIncomingConnection as u8 {
			Self::NewIncomingConnection(LERead::read(reader)?)
		} else if message_id == $crate::raknet::server::MessageId::UserMessage as u8 {
			Self::UserMessage(LERead::read(reader)?)
		} else {
			return err("message id", message_id);
		})
	}
}
}
}

#[derive(Debug)]
pub struct InternalPing {
	pub send_time: u32
}

impl<R: LERead> Deserialize<LE, R> for InternalPing
	where u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		Ok(Self { send_time: reader.read()? })
	}
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

#[derive(Debug)]
pub struct NewIncomingConnection {
	peer_addr: SystemAddress,
	local_addr: SystemAddress,
}

impl<R: LERead> Deserialize<LE, R> for NewIncomingConnection
	where SystemAddress: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let peer_addr = reader.read()?;
		let local_addr = reader.read()?;
		Ok(Self { peer_addr, local_addr })
	}
}
