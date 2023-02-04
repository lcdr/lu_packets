//! Server-received auth messages.
use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;

use crate::common::{LuWString33, LuWString41, LuWString128, LuWString256, ServiceId};
pub use crate::general::server::GeneralMessage;

/// All messages that can be received by an auth server.
pub type Message = crate::raknet::server::Message<LuMessage>;

/// All LU messages that can be received by an auth server.
#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[non_exhaustive]
#[repr(u16)]
pub enum LuMessage {
	General(GeneralMessage) = ServiceId::General as u16,
	Auth(AuthMessage) = ServiceId::Auth as u16,
}

/// All server-received auth messages.
#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding = 1]
#[repr(u32)]
pub enum AuthMessage {
	LoginRequest(LoginRequest),
}

/**
	Provides username and password to authenticate the client.

	Also provides system stats for analytics.

	### Trigger
	Receipt of [`Handshake`](crate::general::client::Handshake) with a [`service_id`](crate::general::client::Handshake`::service_id) of [`ServiceId::Auth`].

	### Handling
	Look up the username and a hashed form of the password in the database. If there is a match, generate a session key to be used as an auth token when the client connects to world servers, and save it to DB. Generate session keys in a way that makes it impossible for an attacker to guess a session key with non-neglible probability. Specifically, this means that using an incrementing number or a non-cryptographically-secure pseudo-random number generator for session keys is ***not*** secure.

	In addition, determine the address of a char server to redirect the client to.

	### Response
	Respond with [`LoginResponse`](super::client::LoginResponse), using an appropriate variant to indicate the lookup status, passing the session key and redirect address if successful.

	### Notes
	The password is provided in plain text. **Don't** save this password to the database unprocessed, as this constitutes a **security hazard**. Hash and salt it using a strong cryptographic hash function before saving it.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LoginRequest {
	/// The client's user name.
	pub username: LuWString33,
	/// The client's password.
	pub password: LuWString41,
	/// The client's locale.
	pub locale_id: u16,
	/// The client's operating system.
	pub client_os: ClientOs,
	/// Stats about the computer the client is running on.
	pub computer_stats: ComputerStats,
}

/// The client's operating system.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum ClientOs {
	Unknown,
	Windows,
	MacOs,
}

/// Stats about the computer the client is running on.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ComputerStats {
	pub memory_stats: LuWString256,
	pub video_card_info: LuWString128,
	/// Info about the processor the client is running on. Collected from a [`GetSystemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsysteminfo) call.
	pub processor_info: ProcessorInfo,
	/// Info about the operating system the client is running on. Collected from a [`GetVersionEx`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getversionexa) call.
	pub os_info: OsInfo,
}

/// Info about the processor the client is running on.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ProcessorInfo {
	/// Number of processors. [`SYSTEM_INFO::dwNumberOfProcessors`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info)
	pub number_of_processors: u32,
	/// Processor type. [`SYSTEM_INFO::dwProcessorType`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info)
	pub processor_type: u32,
	/// Processor level. [`SYSTEM_INFO::wProcessorLevel`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info)
	pub processor_level: u16,
	/// Processor revision. [`SYSTEM_INFO::wProcessorRevision`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info)
	pub processor_revision: u16,
}

/// Info about the operating system the client is running on.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OsInfo {
	/// Size of [`OSVERSIONINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoa). Pretty useless.
	pub os_version_info_size: u32,
	/// OS major version. [`OSVERSIONINFO::dwMajorVersion`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoa).
	pub major_version: u32,
	/// OS minor version. [`OSVERSIONINFO::dwMinorVersion`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoa).
	pub minor_version: u32,
	/// OS build number. [`OSVERSIONINFO::dwBuildNumber`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoa).
	pub build_number: u32,
	/// OS platform ID. [`OSVERSIONINFO::dwPlatformId`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoa).
	pub platform_id: u32,
}
