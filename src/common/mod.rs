mod str;

use std::io::{Error, ErrorKind::InvalidData};
use std::io::Result as Res;
use std::net::Ipv4Addr;

use endio::{Deserialize, Serialize};

pub use self::str::*;

pub(crate) fn err<T, U: std::fmt::Debug>(name: &str, value: U) -> Res<T> {
	Err(Error::new(InvalidData, &format!("unknown {} {:?}", name, value)[..]))
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SystemAddress {
	pub ip: Ipv4Addr,
	pub port: u16,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[repr(u16)]
pub enum ServiceId {
	General = 0,
	Auth = 1,
	Chat = 2,
	World = 4,
	Client = 5,
}

pub type ObjId = u64;
pub const OBJID_EMPTY: u64 = 0;
