//! Server-received general messages.
use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;

use crate::common::ServiceId;

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding = 1]
#[repr(u32)]
pub enum GeneralMessage {
    Handshake(Handshake),
}

/**
    Provides the client's network version.

    Allows to identify outdated clients and disconnect them early.

    ### Trigger
    Establishment of raknet connection (receipt of [`ConnectionRequestAccepted`](crate::raknet::client::ConnectionRequestAccepted)).

    ### Handling
    Check if [`network_version`](Self::network_version) matches the version you expect. Otherwise, disconnect the client, ideally with a [`DisconnectNotify::InvalidGameVersion`](super::client::DisconnectNotify::WrongGameVersion) specifying the expected version.

    ### Response
    Respond with a server-sent [`Handshake`](super::client::Handshake) providing the server's network version and service ID.

    ### Notes
    This packet should not be seen as proof that the client's network version is actually what they report it to be. The client can provide any value, and malicious clients can deviate from the protocol in any way they like. Therefore, proper length and value checking is still required for packet parsing, and care should be taken that your server does not crash on invalid input. If you're using the parsing functionality of this library, this will be taken care of for you.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding = 33]
pub struct Handshake {
    /// The network protocol version of the client. For unmodified live clients, this is `171022`. This was relevant mainly back when LU was actively updated. If you intend to make modifications to the protocol for your server project, you should change this to a different value.
    pub network_version: u32,
    #[padding = 4]
    /// Service ID of the client, always [`ServiceId::Client`]. LU used this packet for all service communications, including server-to-server, which is the reason it's necessary to specify this.
    pub service_id: ServiceId,
    #[padding = 2]
    /// Process ID of the client.
    pub process_id: u32,
    /// Local port of the client, not necessarily the same as the one the connection is from in case of NAT.
    pub port: u16,
}
