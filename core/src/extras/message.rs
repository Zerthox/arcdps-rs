use chrono::{DateTime, FixedOffset};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatMessageInfoOwned {
    pub channel_id: u32,
    pub channel_type: ChannelType,
    pub subgroup: u8,
    pub is_broadcast: bool,
    pub timestamp: DateTime<FixedOffset>,
    pub account_name: String,
    pub character_name: String,
    pub text: String,
}

impl From<ChatMessageInfo<'_>> for ChatMessageInfoOwned {
    fn from(chat: ChatMessageInfo<'_>) -> Self {
        Self {
            channel_id: chat.channel_id,
            channel_type: chat.channel_type,
            subgroup: chat.subgroup,
            is_broadcast: chat.is_broadcast,
            timestamp: chat.timestamp,
            account_name: chat.account_name.to_string(),
            character_name: chat.character_name.to_string(),
            text: chat.text.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessageInfo<'a> {
    pub channel_id: u32,
    pub channel_type: ChannelType,
    pub subgroup: u8,
    pub is_broadcast: bool,
    pub timestamp: DateTime<FixedOffset>,
    pub account_name: &'a str,
    pub character_name: &'a str,
    pub text: &'a str,
}

impl From<RawChatMessageInfo> for ChatMessageInfo<'_> {
    fn from(raw: RawChatMessageInfo) -> Self {
        let timestamp = unsafe { get_str_from_ptr_and_len(raw.timestamp, raw.timestamp_length) };
        let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap();

        let account_name =
            unsafe { get_str_from_ptr_and_len(raw.account_name, raw.account_name_length) };
        let character_name =
            unsafe { get_str_from_ptr_and_len(raw.character_name, raw.character_name_length) };
        let text = unsafe { get_str_from_ptr_and_len(raw.text, raw.text_length) };
        let is_broadcast = (raw.is_broadcast & 0x01) != 0;

        Self {
            channel_id: raw.channel_id,
            channel_type: raw.channel_type,
            subgroup: raw.subgroup,
            is_broadcast,
            timestamp,
            account_name: account_name.trim_start_matches(':'),
            character_name,
            text,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawChatMessageInfo {
    /// A unique identifier for the channel this chat message was sent over. Can
    /// be used to, for example, differentiate between squad messages sent to
    /// different squads
    pub channel_id: u32,

    /// Whether the message is sent in a party or a squad. Note that messages
    /// sent to the party chat while in a squad will have the type
    /// ChannelType::Squad
    pub channel_type: ChannelType,

    /// The subgroup the message was sent to, or 0 if it was sent to the entire
    /// squad.
    pub subgroup: u8,

    /// This lowest bit of this field will be set to 1 if the message is a
    /// broadcast, and 0 if it is not a broadcast. The upper bits of this field
    /// may be used in a later version and MUST NOT be interpreted
    pub is_broadcast: u8,

    pub _unused1: u8,

    /// Null terminated iso8601 formatted string denoting when this message was
    /// received by the server, e.g. "2022-07-09T11:45:24.888Z". This is the
    /// "absolute ordering" for chat messages, however the time can potentially
    /// differ several seconds between the client and server because of latency
    /// and clock skew. The string is only valid for the duration of the call.
    pub timestamp: *const u8,
    pub timestamp_length: u64,

    /// Null terminated account name of the player that sent the message,
    /// including leading ':'. The string is only valid for the duration of the
    /// call.
    pub account_name: *const u8,
    pub account_name_length: u64,

    /// Null terminated character name of the player that sent the message. The
    /// string is only valid for the duration of the call.
    pub character_name: *const u8,
    pub character_name_length: u64,

    /// Null terminated string containing the content of the message that was
    /// sent. The string is only valid for the duration of the call.
    pub text: *const u8,
    pub text_length: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum ChannelType {
    Party = 0,
    Squad = 1,
    Reserved = 2,
    Invalid = 3,
}

/// Converts a pointer and length into a &str with a lifetime. The pointer must not be null
#[inline(always)]
unsafe fn get_str_from_ptr_and_len(src: *const u8, len: u64) -> &'static str {
    let buff = std::slice::from_raw_parts(src, len as usize);
    std::str::from_utf8_unchecked(buff)
}