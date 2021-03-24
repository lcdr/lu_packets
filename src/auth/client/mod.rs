//! Client-received auth messages.
use std::io::Result as Res;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::MessageFromVariants;

use crate::common::{LuString33, LuVarWString, LuWString33, ServiceId};
use crate::general::client::{DisconnectNotify, Handshake, GeneralMessage};

/// All messages that can be received by a client from an auth server.
pub type Message = crate::raknet::client::Message<LuMessage>;

/// All LU messages that can be received by a client from an auth server.
#[derive(Debug, MessageFromVariants, PartialEq, Serialize)]
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

/// All client-received auth messages.
#[derive(Debug, MessageFromVariants, PartialEq, Serialize)]
#[post_disc_padding=1]
#[repr(u32)]
pub enum ClientMessage {
	LoginResponse(LoginResponse),
}

/**
	Responds to a login request.

	Sends session key and redirect address in case of success.

	### Trigger
	Receipt of [`LoginRequest`](super::server::LoginRequest).

	### Handling
	If the variant is not [`Ok`](LoginResponse::Ok), report the error to the user.

	If the variant is `Ok`, store the [`session_key`](LoginResponse::Ok::session_key) for later use. Close the connection and open a connection to [`redirect_address`](LoginResponse::Ok::redirect_address).

	### Response
	None, close the connection.

	### Notes
	Expect the connection to be closed soon after this message is received, if you're not closing it yourself already.
*/
#[derive(Debug, PartialEq)]
#[non_exhaustive]
#[repr(u8)]
pub enum LoginResponse {
	/// The login was successful.
	Ok {
		/// The session key to be used for authenticating with world servers (to be passed in [`ClientValidation::session_key`](crate::world::server::ClientValidation::session_key)).
		session_key: LuWString33,
		/// The address of a world server available for further service.
		redirect_address: (LuString33, u16),
	} = 1,
	/// The login failed in an unusual way. More information can be found in the attached message.
	CustomMessage(LuVarWString<u16>) = 5,
	/// Username or password was incorrect.
	InvalidUsernamePassword = 6,
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a LoginResponse
	where       u8: Serialize<LE, W>,
	           u16: Serialize<LE, W>,
	           u32: Serialize<LE, W>,
	      &'a [u8]: Serialize<LE, W>,
	   &'a LuString33: Serialize<LE, W>,
	  &'a LuWString33: Serialize<LE, W>,
	  &'a LuVarWString<u16>: Serialize<LE, W> {
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
