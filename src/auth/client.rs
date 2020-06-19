use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::{FromVariants, ServiceMessageS};

use crate::common::{LuStr33, LuVarWStr, LuWStr33};

pub type LuMessage = crate::general::client::LuMessage<ClientMessage>;
pub type Message = crate::raknet::client::Message<LuMessage>;

impl From<ClientMessage> for Message {
	fn from(msg: ClientMessage) -> Self {
		LuMessage::Client(msg).into()
	}
}

#[derive(Debug, FromVariants, ServiceMessageS)]
#[non_exhaustive]
#[repr(u32)]
pub enum ClientMessage {
	LoginResponse(LoginResponse),
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum LoginResponse {
	Ok {
		session_key: LuWStr33,
		redirect_address: (LuStr33, u16),
	} = 1,
	CustomMessage(LuVarWStr<u16>) = 5,
	InvalidUsernamePassword = 6,
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a LoginResponse
	where       u8: Serialize<LE, W>,
	           u16: Serialize<LE, W>,
	           u32: Serialize<LE, W>,
	      &'a [u8]: Serialize<LE, W>,
	   &'a LuStr33: Serialize<LE, W>,
	  &'a LuWStr33: Serialize<LE, W>,
	  &'a LuVarWStr<u16>: Serialize<LE, W> {
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
			}
			LoginResponse::CustomMessage(msg) => {
				writer.write(&[0; 493][..])?;
				writer.write(msg)?;
			}
			LoginResponse::InvalidUsernamePassword => {
				writer.write(&[0; 495][..])?;
			}
		}
		writer.write(4u32)?;
		Ok(())
	}
}
