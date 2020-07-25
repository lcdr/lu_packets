pub mod client;
pub mod server;

use std::net::Ipv4Addr;

use endio::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SystemAddress {
	pub ip: Ipv4Addr,
	pub port: u16,
}
