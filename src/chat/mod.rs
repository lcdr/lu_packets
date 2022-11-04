//! Chat messages.
pub mod client;
pub mod server;

use std::io::Result as Res;
use std::io::{Read, Write};

use endio::LittleEndian as LE;
use endio::{Deserialize, LERead, LEWrite, Serialize};

use crate::common::{LuVarWString, LuWString33, ObjId};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum ChatChannel {
    SystemNotify,
    SystemWarning,
    SystemError,
    Broadcast,
    Local,
    LocalNoanim,
    Emote,
    Private,
    Team,
    TeamLocal,
    Guild,
    GuildNotify,
    Property,
    Admin,
    CombatDamage,
    CombatHealing,
    CombatLoot,
    CombatExp,
    CombatDeath,
    General,
    Trade,
    Lfg,
    User,
}

#[derive(Debug, PartialEq)]
pub struct GeneralChatMessage {
    pub chat_channel: ChatChannel,
    pub sender_name: LuWString33,
    pub sender: ObjId,
    pub source_id: u16,
    pub sender_gm_level: u8,
    pub message: LuVarWString<u32>,
}

impl<R: Read + LERead> Deserialize<LE, R> for GeneralChatMessage
where
    u8: Deserialize<LE, R>,
    u16: Deserialize<LE, R>,
    u32: Deserialize<LE, R>,
    LuWString33: Deserialize<LE, R>,
    ObjId: Deserialize<LE, R>,
{
    fn deserialize(reader: &mut R) -> Res<Self> {
        let chat_channel = LERead::read(reader)?;
        let mut str_len: u32 = LERead::read(reader)?;
        if chat_channel == ChatChannel::Team {
            str_len -= 1;
        }
        let sender_name = LERead::read(reader)?;
        let sender = LERead::read(reader)?;
        let source_id = LERead::read(reader)?;
        let sender_gm_level = LERead::read(reader)?;
        let message = LuVarWString::deser_content(reader, str_len)?;
        let _: u16 = LERead::read(reader)?;
        Ok(Self {
            chat_channel,
            sender_name,
            sender,
            source_id,
            sender_gm_level,
            message,
        })
    }
}

impl<'a, W: Write + LEWrite> Serialize<LE, W> for &'a GeneralChatMessage
where
    u8: Serialize<LE, W>,
    u16: Serialize<LE, W>,
    u32: Serialize<LE, W>,
    &'a LuWString33: Serialize<LE, W>,
    ObjId: Serialize<LE, W>,
{
    fn serialize(self, writer: &mut W) -> Res<()> {
        LEWrite::write(writer, &self.chat_channel)?;
        let mut str_len = self.message.len();
        if self.chat_channel == ChatChannel::Team {
            str_len += 1;
        }
        LEWrite::write(writer, str_len as u32)?;
        LEWrite::write(writer, &self.sender_name)?;
        LEWrite::write(writer, self.sender)?;
        LEWrite::write(writer, self.source_id)?;
        LEWrite::write(writer, self.sender_gm_level)?;
        self.message.ser_content(writer)?;
        LEWrite::write(writer, 0u16)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum PrivateChatMessageResponseCode {
    Sent,
    NotOnline,
    GeneralError,
    ReceivedNewWhisper,
    NotFriends,
    SenderFreeTrial,
    ReceiverFreeTrial,
}

#[derive(Debug, PartialEq)]
pub struct PrivateChatMessage {
    pub chat_channel: ChatChannel,
    pub sender_name: LuWString33,
    pub sender: ObjId,
    pub source_id: u16,
    pub sender_gm_level: u8,
    pub recipient_name: LuWString33,
    pub recipient_gm_level: u8,
    pub response_code: PrivateChatMessageResponseCode,
    pub message: LuVarWString<u32>,
}

impl<R: Read + LERead> Deserialize<LE, R> for PrivateChatMessage
where
    u8: Deserialize<LE, R>,
    u16: Deserialize<LE, R>,
    u32: Deserialize<LE, R>,
    LuWString33: Deserialize<LE, R>,
    ObjId: Deserialize<LE, R>,
    PrivateChatMessageResponseCode: Deserialize<LE, R>,
{
    fn deserialize(reader: &mut R) -> Res<Self> {
        let chat_channel = LERead::read(reader)?;
        let mut str_len: u32 = LERead::read(reader)?;
        str_len -= 1;
        let sender_name = LERead::read(reader)?;
        let sender = LERead::read(reader)?;
        let source_id = LERead::read(reader)?;
        let sender_gm_level = LERead::read(reader)?;
        let recipient_name = LERead::read(reader)?;
        let recipient_gm_level = LERead::read(reader)?;
        let response_code = LERead::read(reader)?;
        let message = LuVarWString::deser_content(reader, str_len)?;
        let _: u16 = LERead::read(reader)?;
        Ok(Self {
            chat_channel,
            sender_name,
            sender,
            source_id,
            sender_gm_level,
            recipient_name,
            recipient_gm_level,
            response_code,
            message,
        })
    }
}
impl<'a, W: Write + LEWrite> Serialize<LE, W> for &'a PrivateChatMessage
where
    u8: Serialize<LE, W>,
    u16: Serialize<LE, W>,
    u32: Serialize<LE, W>,
    &'a LuWString33: Serialize<LE, W>,
    ObjId: Serialize<LE, W>,
    &'a PrivateChatMessageResponseCode: Serialize<LE, W>,
{
    fn serialize(self, writer: &mut W) -> Res<()> {
        LEWrite::write(writer, &self.chat_channel)?;
        let mut str_len = self.message.len();
        str_len += 1;
        LEWrite::write(writer, str_len as u32)?;
        LEWrite::write(writer, &self.sender_name)?;
        LEWrite::write(writer, self.sender)?;
        LEWrite::write(writer, self.source_id)?;
        LEWrite::write(writer, self.sender_gm_level)?;
        LEWrite::write(writer, &self.recipient_name)?;
        LEWrite::write(writer, self.recipient_gm_level)?;
        LEWrite::write(writer, &self.response_code)?;
        self.message.ser_content(writer)?;
        LEWrite::write(writer, 0u16)
    }
}
