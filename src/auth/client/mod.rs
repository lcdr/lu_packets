//! Client-received auth messages.
use std::io::{Error, ErrorKind::InvalidData, Result as Res, Read};

use endio::{LEWrite, LERead, Deserialize, Serialize};
use endio::LittleEndian as LE;
use lu_packets_derive::MessageFromVariants;
use lu_packets_derive::VariantTests;

use crate::common::{LuString3, LuString33, LuString37, LuVarWString, LuWString33, ServiceId};
use crate::general::client::{DisconnectNotify, Handshake, GeneralMessage};
use crate::world::server::Language;

/// All messages that can be received by a client from an auth server.
pub type Message = crate::raknet::client::Message<LuMessage>;

/// All LU messages that can be received by a client from an auth server.
#[derive(Debug, MessageFromVariants, PartialEq, Serialize, Deserialize, VariantTests)]
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
#[derive(Debug, MessageFromVariants, PartialEq, Serialize, Deserialize)]
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
		// Strings used for event gating.
		events: (LuString33, LuString33, LuString33, LuString33, LuString33, LuString33, LuString33, LuString33),
		/// Used for version gating.
		version: (u16, u16, u16),
		/// The session key to be used for authenticating with world servers (to be passed in [`ClientValidation::session_key`](crate::world::server::ClientValidation::session_key)).
		session_key: LuWString33,
		/// The address of a world server available for further service.
		redirect_address: (LuString33, u16),
		/// The address of the chat server (unused).
		chat_server_address: (LuString33, u16),
		cdn_key: LuString33,
		cdn_ticket: LuString37,
		/// Language of the server.
		language: Language,
		/// Used for the cdclient SubscriptionPricing table.
		country_code: LuString3,
		/// Whether the account is connecting as a paid account for the first time.
		just_upgraded_from_ftp: bool,
		/// Whether the account is in free trial mode.
		is_ftp: bool,
		/// Time remaining in seconds for free-to-play (unused).
		time_remaining_in_ftp: u64,
		/// Logs from the auth server.
		stamps: Vec<Stamp>,
	} = 1,
	/// The login failed in an unusual way. More information can be found in the attached message.
	CustomMessage(LuVarWString<u16>) = 5,
	/// Username or password was incorrect.
	InvalidUsernamePassword = 6,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stamp {
	pub type_: u32,
	pub value: u32,
	pub timestamp: u64,
}

impl<'a, W: LEWrite> Serialize<LE, W> for &'a LoginResponse
where
	u8: Serialize<LE, W>,
	u16: Serialize<LE, W>,
	u32: Serialize<LE, W>,
	u64: Serialize<LE, W>,
	&'a bool: Serialize<LE, W>,
	&'a [u8]: Serialize<LE, W>,
	&'a LuString33: Serialize<LE, W>,
	&'a LuString33: Serialize<LE, W>,
	&'a LuWString33: Serialize<LE, W>,
	&'a LuString37: Serialize<LE, W>,
	&'a LuString3: Serialize<LE, W>,
	&'a Language: Serialize<LE, W>,
	&'a Stamp: Serialize<LE, W>,
	&'a LuVarWString<u16>: Serialize<LE, W>,
{
	fn serialize(self, writer: &mut W) -> Res<()> {
		let disc = unsafe { *(self as *const LoginResponse as *const u8) };
		writer.write(disc)?;
		match self {
			LoginResponse::Ok {
				events,
				version,
				session_key,
				redirect_address,
				chat_server_address,
				cdn_key,
				cdn_ticket,
				language,
				country_code,
				just_upgraded_from_ftp,
				is_ftp,
				time_remaining_in_ftp,
				stamps,
			} => {
				writer.write(&events.0)?;
				writer.write(&events.1)?;
				writer.write(&events.2)?;
				writer.write(&events.3)?;
				writer.write(&events.4)?;
				writer.write(&events.5)?;
				writer.write(&events.6)?;
				writer.write(&events.7)?;
				writer.write(version.0)?;
				writer.write(version.1)?;
				writer.write(version.2)?;
				writer.write(session_key)?;
				writer.write(&redirect_address.0)?;
				writer.write(&chat_server_address.0)?;
				writer.write(redirect_address.1)?;
				writer.write(chat_server_address.1)?;
				writer.write(cdn_key)?;
				writer.write(cdn_ticket)?;
				writer.write(language)?;
				writer.write(country_code)?;
				writer.write(just_upgraded_from_ftp)?;
				writer.write(is_ftp)?;
				writer.write(*time_remaining_in_ftp)?;
				// custom message
				writer.write(0u16)?;
				writer.write((stamps.len() * 16) as u32 + 4)?;
				for stamp in stamps {
					writer.write(stamp)?;
				}
			}
			LoginResponse::CustomMessage(msg) => {
				writer.write(&[0; 493][..])?;
				writer.write(msg)?;
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

impl<R: Read + LERead> Deserialize<LE, R> for LoginResponse {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let disc = LERead::read::<u8>(reader)?;
		match disc {
			1 => {
				let events: (LuString33, LuString33, LuString33, LuString33, LuString33, LuString33, LuString33, LuString33) 
					= (LERead::read(reader)?, LERead::read(reader)?, LERead::read(reader)?, LERead::read(reader)?, 
					LERead::read(reader)?, LERead::read(reader)?, LERead::read(reader)?, LERead::read(reader)?);
				let version: (u16, u16, u16) = (
					LERead::read(reader)?,
					LERead::read(reader)?,
					LERead::read(reader)?,
				);
				let session_key: LuWString33 = LERead::read(reader)?;
				let redirect_address: LuString33 = LERead::read(reader)?;
				let chat_address: LuString33 = LERead::read(reader)?;
				let redirect_port: u16 = LERead::read(reader)?;
				let chat_port: u16 = LERead::read(reader)?;
				let cdn_key: LuString33 = LERead::read(reader)?;
				let cdn_ticket: LuString37 = LERead::read(reader)?;
				let language: Language = LERead::read(reader)?;
				let country_code: LuString3 = LERead::read(reader)?;
				let just_upgraded_from_ftp: bool = LERead::read(reader)?;
				let is_ftp: bool = LERead::read(reader)?;
				let time_remaining_in_ftp: u64 = LERead::read(reader)?;
				let _custom_message: LuVarWString<u16> = LERead::read(reader)?;
				let buffer_len_plus_four: u32 = LERead::read(reader)?;
				let mut stamps: Vec<Stamp> = Vec::new();
				let stamp_count = (buffer_len_plus_four - 4) / 16;
				for _i in 0..stamp_count {
					let stamp: Stamp = LERead::read(reader)?;
					stamps.push(stamp);
				}
				Ok(Self::Ok {
					events,
					version,
					session_key,
					redirect_address: (redirect_address, redirect_port),
					chat_server_address: (chat_address, chat_port),
					cdn_key,
					cdn_ticket,
					language,
					country_code,
					just_upgraded_from_ftp,
					is_ftp,
					time_remaining_in_ftp,
					stamps,
				})
			}
			5 => {
				let mut padding = [0; 493];
				Read::read_exact(reader, &mut padding)?;
				let msg = LERead::read::<LuVarWString<u16>>(reader)?;
				Ok(Self::CustomMessage(msg))
			}
			6 => {
				let mut padding = [0; 495];
				Read::read_exact(reader, &mut padding)?;
				Ok(Self::InvalidUsernamePassword)
			}
			_ => Err(Error::new(InvalidData, "invalid login response type")),
		}
	}
}
