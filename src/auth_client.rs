use std::io::Result as Res;
use std::net::Ipv4Addr;

use endio::{LEWrite, Serialize};
use endio::LittleEndian as LE;

use crate::common::{LuStr33, LuWStr33, ServiceId};

enum MessageId {
	ConnectedPong = 3,
	ConnectionRequestAccepted = 14,
	UserMessage = 83,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Message {
	ConnectedPong { ping_send_time: u32 },
	ConnectionRequestAccepted { peer_ip: Ipv4Addr, peer_port: u16, local_ip: Ipv4Addr, local_port: u16 },
	UserMessage(LUMessage),
}

impl<W: LEWrite> Serialize<LE, W> for &Message
	where                 u8: Serialize<LE, W>,
	                     u16: Serialize<LE, W>,
	                     u32: Serialize<LE, W>,
	        for<'b> &'b [u8]: Serialize<LE, W>,
	for<'c> &'c LUMessage: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			Message::ConnectedPong { ping_send_time } => {
				LEWrite::write(writer, MessageId::ConnectedPong as u8)?;
				LEWrite::write(writer, *ping_send_time)?;
			}
			Message::ConnectionRequestAccepted { peer_ip, peer_port, local_ip, local_port } => {
				LEWrite::write(writer, MessageId::ConnectionRequestAccepted as u8)?;
				LEWrite::write(writer, &peer_ip.octets()[..])?;
				LEWrite::write(writer, *peer_port)?;
				LEWrite::write(writer, &[0u8; 2][..])?;
				LEWrite::write(writer, &local_ip.octets()[..])?;
				LEWrite::write(writer, *local_port)?;
			}
			Message::UserMessage(message) => {
				LEWrite::write(writer, MessageId::UserMessage as u8)?;
				LEWrite::write(writer, message)?;
			}
		}
		Ok(())
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

enum GeneralId {
	Handshake = 0,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum GeneralMessage {
	Handshake(Handshake)
}

impl From<GeneralMessage> for Message {
	fn from(msg: GeneralMessage) -> Self {
		LUMessage::General(msg).into()
	}
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a GeneralMessage
	where u8: Serialize<LE, W>,
	     u32: Serialize<LE, W>,
	     &'a Handshake: Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		match self {
			GeneralMessage::Handshake(handshake) => {
				writer.write(GeneralId::Handshake as u32)?;
				writer.write(0u8)?;
				writer.write(handshake)?;
			}
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct Handshake {
	pub network_version: u32,
	pub service_id: ServiceId,
}

impl From<Handshake> for Message {
	fn from(msg: Handshake) -> Self {
		GeneralMessage::Handshake(msg).into()
	}
}

impl<W: LEWrite> Serialize<LE, W> for &Handshake
	where u8 : Serialize<LE, W>,
	     u16 : Serialize<LE, W>,
	     u32 : Serialize<LE, W> {
	fn serialize(self, writer: &mut W) -> Res<()>	{
		writer.write(self.network_version)?;
		writer.write(0u32)?;
		writer.write(self.service_id)?;
		writer.write(0u16)?;
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
