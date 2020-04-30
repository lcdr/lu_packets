//! All packets a world server can receive.
use std::io::Read;
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;

use crate::common::{err, LuWStr33, LuStr33, ServiceId};
pub use crate::general::server::GeneralMessage;

rak_server_msg!(LUMessage);

#[derive(Debug)]
#[non_exhaustive]
pub enum LUMessage {
	General(GeneralMessage),
	World(WorldMessage),
}

impl<R: LERead> Deserialize<LE, R> for LUMessage
	where        u16: Deserialize<LE, R>,
	  GeneralMessage: Deserialize<LE, R>,
	     WorldMessage: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let service_id: ServiceId = reader.read()?;
		Ok(match service_id {
			ServiceId::General => {
				LUMessage::General(reader.read()?)
			}
			ServiceId::World => {
				LUMessage::World(reader.read()?)
			}
			_ => {
				return err("invalid service id");
			}
		})
	}
}

enum WorldId {
	ClientValidation = 1,
	CharacterListRequest = 2,
}

#[derive(Debug)]
pub enum WorldMessage {
	ClientValidation(ClientValidation),
	CharacterListRequest,
}

impl<R: LERead> Deserialize<LE, R> for WorldMessage
	where           u8: Deserialize<LE, R>,
	               u32: Deserialize<LE, R>,
	  ClientValidation: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let packet_id: u32 = reader.read()?;
		let _padding: u8   = reader.read()?;
		if packet_id == WorldId::ClientValidation as u32 {
			Ok(WorldMessage::ClientValidation(reader.read()?))
		} else if packet_id == WorldId::CharacterListRequest as u32 {
			Ok(WorldMessage::CharacterListRequest)
		} else {
			err("invalid world id")
		}
	}
}

#[derive(Debug)]
pub struct ClientValidation {
	pub username: LuWStr33,
	pub session_key: LuWStr33,
	pub fdb_checksum: [u8; 32],
}

impl<R: Read+LERead> Deserialize<LE, R> for ClientValidation
	where LuWStr33: Deserialize<LE, R>,
	       LuStr33: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let username             = LERead::read(reader)?;
		let session_key          = LERead::read(reader)?;
		let mut fdb_checksum = [0; 32];
		std::io::Read::read(reader, &mut fdb_checksum)?;
		// garbage byte because the devs messed up the null terminator
		let _ : u8               =  LERead::read(reader)?;
		Ok(Self {
			username,
			session_key,
			fdb_checksum,
		})
	}
}
