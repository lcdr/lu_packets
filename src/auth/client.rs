use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

use crate::common::{LuStr33, LuWStr33, ServiceId};
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
	LoginResponse = 0,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ClientMessage {
	LoginResponse(LoginResponse),
}

impl From<ClientMessage> for Message {
	fn from(msg: ClientMessage) -> Self {
		LUMessage::Client(msg).into()
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a ClientMessage
	where            u8: Serialize<LE, W>,
	                u32: Serialize<LE, W>,
	  &'a LoginResponse: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			ClientMessage::LoginResponse(message) => {
				writer.write(ClientId::LoginResponse as u32)?;
				writer.write(0u8)?;
				writer.write(message)?;
			}
		}
		Ok(())
	}
}

enum LoginResponseId {
	Ok = 1,
	CustomMessage = 5,
	InvalidUsernamePassword = 6,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum LoginResponse {
	Ok {
		session_key: LuWStr33,
		redirect_address: (LuStr33, u16),
	},
	CustomMessage(String),
	InvalidUsernamePassword,
}

impl From<LoginResponse> for Message {
	fn from(msg: LoginResponse) -> Self {
		ClientMessage::LoginResponse(msg).into()
	}
}

impl<W: LEWrite> Serialize<LE, W> for &LoginResponse
	where               u8: Serialize<LE, W>,
	                   u16: Serialize<LE, W>,
	                   u32: Serialize<LE, W>,
	      for<'a> &'a [u8]: Serialize<LE, W>,
	 for<'c> &'c LuStr33: Serialize<LE, W>,
	for<'c> &'c LuWStr33: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			LoginResponse::Ok { session_key, redirect_address } => {
				writer.write(LoginResponseId::Ok as u8)?;
				writer.write(&[0; 264][..])?;
				writer.write(1u16)?;
				writer.write(10u16)?;
				writer.write(64u16)?;
				writer.write(session_key)?;
				writer.write(&redirect_address.0)?;
				writer.write(&[0; 33][..])?;
				writer.write(redirect_address.1)?;
				writer.write(&[0; 91][..])?;
				writer.write(4u32)?;
			}
			LoginResponse::CustomMessage(msg) => {
				writer.write(LoginResponseId::CustomMessage as u8)?;
				writer.write(&[0; 493][..])?;
				let bytes: Vec<u16> = msg.encode_utf16().collect();
				writer.write(bytes.len() as u16)?;
				for wchar in bytes {
					writer.write(wchar)?;
				}
				writer.write(4u32)?;
			}
			LoginResponse::InvalidUsernamePassword => {
				writer.write(LoginResponseId::InvalidUsernamePassword as u8)?;
				writer.write(&[0; 495][..])?;
				writer.write(4u32)?;
			}
		}
		Ok(())
	}
}
