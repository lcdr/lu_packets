use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

use crate::common::{ServiceId};
use crate::general::client::{GeneralMessage, Handshake};

rak_client_msg!(LUMessage);

impl From<GeneralMessage> for Message {
	fn from(msg: GeneralMessage) -> Self {
		LUMessage::General(msg).into()
	}
}

impl From<Handshake> for Message {
	fn from(msg: Handshake) -> Self {
		GeneralMessage::Handshake(msg).into()
	}
}

#[derive(Debug)]
pub enum LUMessage {
	General(GeneralMessage),
	Client(ClientMessage),
}

impl From<LUMessage> for Message {
	fn from(msg: LUMessage) -> Self {
		Message::UserMessage(msg)
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a LUMessage
	where          u16: Serialize<LE, W>,
	&'a GeneralMessage: Serialize<LE, W>,
	 &'a ClientMessage: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			LUMessage::General(message) => {
				writer.write(ServiceId::General as u16)?;
				writer.write(message)?;
			}
			LUMessage::Client(message) => {
				writer.write(ServiceId::Client as u16)?;
				writer.write(message)?;
			}
		}
		Ok(())
	}
}


enum ClientId {
	CharacterListResponse = 6,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ClientMessage {
	CharacterListResponse(CharacterListResponse),
}

impl From<ClientMessage> for Message {
	fn from(msg: ClientMessage) -> Self {
		LUMessage::Client(msg).into()
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a ClientMessage
	where                    u8: Serialize<LE, W>,
	                        u32: Serialize<LE, W>,
	  &'a CharacterListResponse: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			ClientMessage::CharacterListResponse(message) => {
				writer.write(ClientId::CharacterListResponse as u32)?;
				writer.write(0u8)?;
				writer.write(message)?;
			}
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct CharacterListResponse {
	pub selected_char: u8,
	pub chars: Vec<CharListChar>,
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a CharacterListResponse
	where u8: Serialize<LE, W>,
	     &'a CharListChar: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.chars.len() as u8)?;
		writer.write(self.selected_char)?;
		for chr in self.chars.iter() {
			writer.write(chr)?;
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct CharListChar {
	obj_id: u64,
}

impl<W: LEWrite> Serialize<LE, W> for &CharListChar
	where u8: Serialize<LE, W>,
	     u32: Serialize<LE, W>,
	   ObjId: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.obj_id)?;
		writer.write(0u32)?; // unused
		Ok(())
	}
}

type ObjId = u64;
