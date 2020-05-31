use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

use crate::common::{ObjId, LuWStr33, ServiceId, ZoneId};
use crate::general::client::{DisconnectNotify, GeneralMessage, Handshake};

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

impl From<DisconnectNotify> for Message {
	fn from(msg: DisconnectNotify) -> Self {
		GeneralMessage::DisconnectNotify(msg).into()
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
	CharacterCreateResponse = 7,
	CharacterDeleteResponse = 11,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ClientMessage {
	CharacterListResponse(CharacterListResponse),
	CharacterCreateResponse(CharacterCreateResponse),
	CharacterDeleteResponse(CharacterDeleteResponse),
}

impl From<ClientMessage> for Message {
	fn from(msg: ClientMessage) -> Self {
		LUMessage::Client(msg).into()
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a ClientMessage
	where                      u8: Serialize<LE, W>,
	                          u32: Serialize<LE, W>,
	    &'a CharacterListResponse: Serialize<LE, W>,
	  &'a CharacterDeleteResponse: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			ClientMessage::CharacterListResponse(msg) => {
				writer.write(ClientId::CharacterListResponse as u32)?;
				writer.write(0u8)?;
				writer.write(msg)?;
			}
			ClientMessage::CharacterCreateResponse(msg) => {
				writer.write(ClientId::CharacterCreateResponse as u32)?;
				writer.write(0u8)?;
				writer.write(msg)?;
			}
			ClientMessage::CharacterDeleteResponse(msg) => {
				writer.write(ClientId::CharacterDeleteResponse as u32)?;
				writer.write(0u8)?;
				writer.write(msg)?;
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

impl From<CharacterListResponse> for Message {
	fn from(msg: CharacterListResponse) -> Self {
		ClientMessage::CharacterListResponse(msg).into()
	}
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
	pub obj_id: u64,
	pub char_name: LuWStr33,
	pub pending_name: LuWStr33,
	pub requires_rename: bool,
	pub is_ftp: bool,
	pub shirt_color: u32,
	pub pants_color: u32,
	pub hair_style: u32,
	pub hair_color: u32,
	pub eyebrow_style: u32,
	pub eye_style: u32,
	pub mouth_style: u32,
	pub last_location: ZoneId,
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a CharListChar
	where       u8: Serialize<LE, W>,
	           u16: Serialize<LE, W>,
	           u32: Serialize<LE, W>,
	         ObjId: Serialize<LE, W>,
	      &'a [u8]: Serialize<LE, W>,
	  &'a LuWStr33: Serialize<LE, W>,
	          bool: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.obj_id)?;
		writer.write(0u32)?; // unused
		writer.write(&self.char_name)?;
		writer.write(&self.pending_name)?;
		writer.write(self.requires_rename)?;
		writer.write(self.is_ftp)?;
		writer.write(&[0; 10][..])?;

		writer.write(self.shirt_color)?;
		writer.write(&[0; 4][..])?;

		writer.write(self.pants_color)?;
		writer.write(self.hair_style)?;
		writer.write(self.hair_color)?;
		writer.write(&[0; 8][..])?;

		writer.write(self.eyebrow_style)?;
		writer.write(self.eye_style)?;
		writer.write(self.mouth_style)?;
		writer.write(&[0; 4][..])?;

		writer.write(&self.last_location)?;
		writer.write(&[0; 8][..])?;

		writer.write(0u16)?;
		Ok(())
	}
}

#[derive(Clone, Copy, Debug)]
pub enum CharacterCreateResponse {
	Success = 0,
	GeneralFailure = 1,
	NameNotAllowed = 2,
	PredefinedNameInUse = 3,
	CustomNameInUse = 4,
}

impl From<CharacterCreateResponse> for Message {
	fn from(msg: CharacterCreateResponse) -> Self {
		ClientMessage::CharacterCreateResponse(msg).into()
	}
}

impl<W: LEWrite> Serialize<LE, W> for &CharacterCreateResponse
	where u8: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(*self as u8)?;
		Ok(())
	}
}

#[derive(Debug)]
pub struct CharacterDeleteResponse {
	pub success: bool,
}

impl From<CharacterDeleteResponse> for Message {
	fn from(msg: CharacterDeleteResponse) -> Self {
		ClientMessage::CharacterDeleteResponse(msg).into()
	}
}

impl<W: LEWrite> Serialize<LE, W> for &CharacterDeleteResponse
	where bool: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.success)?;
		Ok(())
	}
}
