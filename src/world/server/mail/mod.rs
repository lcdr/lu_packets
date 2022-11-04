use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;

use crate::common::{LuWString32, LuWString400, LuWString50, ObjId};

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[repr(u32)]
pub enum Mail {
    CreateRequest(CreateRequest) = 0,
    ListRequest = 3,
    ContentCollectRequest(ContentCollectRequest) = 5,
    DeleteRequest(DeleteRequest) = 7,
    MarkAsReadRequest(MarkAsReadRequest) = 9,
    UnreadCountRequest = 11,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding = 4]
pub struct CreateRequest {
    pub subject: LuWString50,
    pub body: LuWString400,
    pub receiver_name: LuWString32,
    #[padding = 8] // money: i64, unused
    pub attachment_id: ObjId,
    pub attachment_count: u16,
    pub locale_id: u16,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ContentCollectRequest {
    #[padding = 4]
    pub mail_id: ObjId,
    pub receiver_id: ObjId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct DeleteRequest {
    #[padding = 4]
    pub mail_id: ObjId,
    pub receiver_id: ObjId,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MarkAsReadRequest {
    #[padding = 4]
    pub mail_id: ObjId,
}
