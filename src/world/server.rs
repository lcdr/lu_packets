//! All packets a world server can receive.
use std::io::Read;
use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;

use crate::common::{err, ObjId, LuWStr33, LuStr33, ServiceId};
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
				return err("service id", service_id);
			}
		})
	}
}

enum WorldId {
	ClientValidation = 1,
	CharacterListRequest = 2,
	CharacterCreateRequest = 3,
	CharacterLoginRequest = 4,
	CharacterDeleteRequest = 6,
}

#[derive(Debug)]
pub enum WorldMessage {
	ClientValidation(ClientValidation),
	CharacterListRequest,
	CharacterCreateRequest(CharacterCreateRequest),
	CharacterLoginRequest(CharacterLoginRequest),
	CharacterDeleteRequest(CharacterDeleteRequest),
}

impl<R: LERead> Deserialize<LE, R> for WorldMessage
	where                 u8: Deserialize<LE, R>,
	                     u32: Deserialize<LE, R>,
	        ClientValidation: Deserialize<LE, R>,
	  CharacterCreateRequest: Deserialize<LE, R>,
	  CharacterLoginRequest: Deserialize<LE, R>,
	  CharacterDeleteRequest: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let packet_id: u32 = reader.read()?;
		let _padding: u8   = reader.read()?;
		if packet_id == WorldId::ClientValidation as u32 {
			Ok(WorldMessage::ClientValidation(reader.read()?))
		} else if packet_id == WorldId::CharacterListRequest as u32 {
			Ok(WorldMessage::CharacterListRequest)
		} else if packet_id == WorldId::CharacterCreateRequest as u32 {
			Ok(WorldMessage::CharacterCreateRequest(reader.read()?))
		} else if packet_id == WorldId::CharacterLoginRequest as u32 {
			Ok(WorldMessage::CharacterLoginRequest(reader.read()?))
		} else if packet_id == WorldId::CharacterDeleteRequest as u32 {
			Ok(WorldMessage::CharacterDeleteRequest(reader.read()?))
		} else {
			err("world id", packet_id)
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

#[derive(Debug)]
pub struct CharacterCreateRequest {
	pub char_name: LuWStr33,
	pub predef_name_ids: (u32, u32, u32),
	pub shirt_color: u32,
	pub pants_color: u32,
	pub hair_style: u32,
	pub hair_color: u32,
	pub eyebrow_style: u32,
	pub eye_style: u32,
	pub mouth_style: u32,
}

impl<R: LERead> Deserialize<LE, R> for CharacterCreateRequest
	where  u8: Deserialize<LE, R>,
	      u32: Deserialize<LE, R>,
	 LuWStr33: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let char_name = reader.read()?;
		let name_id_1 = reader.read()?;
		let name_id_2 = reader.read()?;
		let name_id_3 = reader.read()?;
		let predef_name_ids = (name_id_1, name_id_2, name_id_3);
		let _unused: u8   = reader.read()?;
		let _unused: u32  = reader.read()?;
		let _unused: u32  = reader.read()?;
		let shirt_color   = reader.read()?;
		let _unused: u32  = reader.read()?;
		let pants_color   = reader.read()?;
		let hair_style    = reader.read()?;
		let hair_color    = reader.read()?;
		let _unused: u32  = reader.read()?;
		let _unused: u32  = reader.read()?;
		let eyebrow_style = reader.read()?;
		let eye_style     = reader.read()?;
		let mouth_style   = reader.read()?;
		let _unused: u8  = reader.read()?;

		Ok(Self {
			char_name,
			predef_name_ids,
			shirt_color,
			pants_color,
			hair_style,
			hair_color,
			eyebrow_style,
			eye_style,
			mouth_style,
		})
	}
}

#[derive(Debug)]
pub struct CharacterLoginRequest {
	pub char_id: ObjId,
}

impl<R: LERead> Deserialize<LE, R> for CharacterLoginRequest
	where  ObjId: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let char_id = reader.read()?;

		Ok(Self {
			char_id,
		})
	}
}

#[derive(Debug)]
pub struct CharacterDeleteRequest {
	pub char_id: ObjId,
}

impl<R: LERead> Deserialize<LE, R> for CharacterDeleteRequest
	where  ObjId: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let char_id = reader.read()?;

		Ok(Self {
			char_id,
		})
	}
}
