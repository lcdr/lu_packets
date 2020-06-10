use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

use crate::common::{LuStr33, LuWStr33, ServiceId};
use crate::general::client::{GeneralMessage, Handshake};

rak_client_msg!(LuMessage);

impl From<GeneralMessage> for Message {
	fn from(msg: GeneralMessage) -> Self {
		LuMessage::General(msg).into()
	}
}

impl From<Handshake> for Message {
	fn from(msg: Handshake) -> Self {
		GeneralMessage::Handshake(msg).into()
	}
}

#[derive(Debug, Serialize)]
#[repr(u16)]
pub enum LuMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	Client(ClientMessage) = ServiceId::Client as u16,
}

impl From<LuMessage> for Message {
	fn from(msg: LuMessage) -> Self {
		Message::UserMessage(msg)
	}
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(u32)]
pub enum ClientMessage {
	LoginResponse(LoginResponse),
}

impl From<ClientMessage> for Message {
	fn from(msg: ClientMessage) -> Self {
		LuMessage::Client(msg).into()
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a ClientMessage
	where            u8: Serialize<LE, W>,
	                u32: Serialize<LE, W>,
	  &'a LoginResponse: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		let disc = unsafe { *(self as *const ClientMessage as *const u32) };
		writer.write(disc)?;
		writer.write(0u8)?;
		match self {
			ClientMessage::LoginResponse(message) => {
				writer.write(message)?;
			}
		}
		Ok(())
	}
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum LoginResponse {
	Ok {
		session_key: LuWStr33,
		redirect_address: (LuStr33, u16),
	} = 1,
	CustomMessage(String) = 5,
	InvalidUsernamePassword = 6,
}

impl From<LoginResponse> for Message {
	fn from(msg: LoginResponse) -> Self {
		ClientMessage::LoginResponse(msg).into()
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a LoginResponse
	where       u8: Serialize<LE, W>,
	           u16: Serialize<LE, W>,
	           u32: Serialize<LE, W>,
	      &'a [u8]: Serialize<LE, W>,
	   &'a LuStr33: Serialize<LE, W>,
	  &'a LuWStr33: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		let disc = unsafe { *(self as *const LoginResponse as *const u8) };
		writer.write(disc)?;
		match self {
			LoginResponse::Ok { session_key, redirect_address } => {
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
				writer.write(&[0; 493][..])?;
				let bytes: Vec<u16> = msg.encode_utf16().collect();
				writer.write(bytes.len() as u16)?;
				for wchar in bytes {
					writer.write(wchar)?;
				}
				writer.write(4u32)?;
			}
			LoginResponse::InvalidUsernamePassword => {
				writer.write(&[0; 495][..])?;
				writer.write(4u32)?;
			}
		}
		Ok(())
	}
}
