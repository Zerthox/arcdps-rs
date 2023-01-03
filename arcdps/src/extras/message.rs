use crate::{strip_account_prefix, util::str_from_cstr_len};
use chrono::{DateTime, FixedOffset};
use std::os::raw::c_char;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// A [`ChatMessageInfo`] with owned [`String`] fields.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatMessageInfoOwned {
    /// A unique identifier for the channel this chat message was sent over.
    ///
    /// Can be used to, for example, differentiate between squad messages sent to different squads.
    pub channel_id: u32,

    /// Whether the message is sent in a party or a squad.
    ///
    /// Note that messages sent to the party chat while in a squad will have the type [`ChannelType::Squad`].
    pub channel_type: ChannelType,

    /// The subgroup the message was sent to, or `0` if it was sent to the entire squad.
    pub subgroup: u8,

    /// Whether the message is a broadcast.
    pub is_broadcast: bool,

    /// Timestamp when the message was received.
    ///
    /// This is the "absolute ordering" for chat messages,
    /// however the time can potentially differ several seconds between the client and server because of latency and clock skew.
    pub timestamp: DateTime<FixedOffset>,

    /// Account name of the player that sent the message.
    pub account_name: String,

    /// Character name of the player that sent the message.
    pub character_name: String,

    /// Content of the message.
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

/// A chat message.
///
/// Strings are available for the duration of the call.
/// If you need it for longer than that, consider converting it to [`ChatMessageInfoOwned`].
///
/// ```no_run
/// # use arcdps::extras::message::{ChatMessageInfo, ChatMessageInfoOwned};
/// # fn f(message: ChatMessageInfo) {
/// let message: ChatMessageInfoOwned = message.into();
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ChatMessageInfo<'a> {
    /// A unique identifier for the channel this chat message was sent over.
    ///
    /// Can be used to, for example, differentiate between squad messages sent to different squads.
    pub channel_id: u32,

    /// Whether the message is sent in a party or a squad.
    ///
    /// Note that messages sent to the party chat while in a squad will have the type [`ChannelType::Squad`].
    pub channel_type: ChannelType,

    /// The subgroup the message was sent to, or `0` if it was sent to the entire squad.
    pub subgroup: u8,

    /// Whether the message is a broadcast.
    pub is_broadcast: bool,

    /// Timestamp when the message was received.
    ///
    /// This is the "absolute ordering" for chat messages,
    /// however the time can potentially differ several seconds between the client and server because of latency and clock skew.
    pub timestamp: DateTime<FixedOffset>,

    /// Account name of the player that sent the message.
    pub account_name: &'a str,

    /// Character name of the player that sent the message.
    pub character_name: &'a str,

    /// Content of the message.
    pub text: &'a str,
}

impl From<&RawChatMessageInfo> for ChatMessageInfo<'_> {
    fn from(raw: &RawChatMessageInfo) -> Self {
        let timestamp = unsafe { str_from_cstr_len(raw.timestamp, raw.timestamp_length) };
        let timestamp =
            DateTime::parse_from_rfc3339(timestamp).expect("failed to parse message timestamp");

        let account_name = unsafe { str_from_cstr_len(raw.account_name, raw.account_name_length) };
        let character_name =
            unsafe { str_from_cstr_len(raw.character_name, raw.character_name_length) };
        let text = unsafe { str_from_cstr_len(raw.text, raw.text_length) };

        let is_broadcast = (raw.is_broadcast & 0x01) != 0;

        Self {
            channel_id: raw.channel_id,
            channel_type: raw.channel_type,
            subgroup: raw.subgroup,
            is_broadcast,
            timestamp,
            account_name: strip_account_prefix(account_name),
            character_name,
            text,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawChatMessageInfo {
    /// A unique identifier for the channel this chat message was sent over.
    ///
    /// Can be used to, for example, differentiate between squad messages sent to different squads.
    pub channel_id: u32,

    /// Whether the message is sent in a party or a squad.
    ///
    /// Note that messages sent to the party chat while in a squad will have the type [`ChannelType::Squad`].
    pub channel_type: ChannelType,

    /// The subgroup the message was sent to, or `0` if it was sent to the entire squad.
    pub subgroup: u8,

    /// The lowest bit of this field will be set to `1` if the message is a broadcast, and `0` if it is not a broadcast.
    /// The upper bits of this field may be used in a later version and **must not** be interpreted.
    pub is_broadcast: u8,

    /// Unused padding.
    pub _unused1: u8,

    /// Null terminated iso8601 formatted string denoting when this message was
    /// received by the server, e.g. `"2022-07-09T11:45:24.888Z"`.
    /// This is the "absolute ordering" for chat messages,
    /// however the time can potentially differ several seconds between the client and server because of latency and clock skew.
    ///
    /// The string is only valid for the duration of the call.
    pub timestamp: *const c_char,
    pub timestamp_length: u64,

    /// Null terminated account name of the player that sent the message, including leading ':'.
    ///
    /// The string is only valid for the duration of the call.
    pub account_name: *const c_char,
    pub account_name_length: u64,

    /// Null terminated character name of the player that sent the message.
    ///
    /// The string is only valid for the duration of the call.
    pub character_name: *const c_char,
    pub character_name_length: u64,

    /// Null terminated string containing the content of the message that was sent.
    ///
    /// The string is only valid for the duration of the call.
    pub text: *const c_char,
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
