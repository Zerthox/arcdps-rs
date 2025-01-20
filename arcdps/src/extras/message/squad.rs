use crate::{strip_account_prefix, util::str_from_cstr_len};
use bitflags::bitflags;
use chrono::{DateTime, FixedOffset};
use std::ffi::c_char;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// A squad/party chat message.
///
/// Strings are available for the duration of the call.
/// If you need it for longer than that, consider converting it to [`SquadMessageOwned`].
///
/// ```no_run
/// # use arcdps::extras::{SquadMessage, SquadMessageOwned};
/// # let message: &SquadMessage = todo!();
/// let owned = message.to_owned();
/// let owned: SquadMessageOwned = message.into();
/// ```
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SquadMessage {
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
    flags: u8,

    /// Unused padding.
    _unused1: u8,

    /// Null terminated iso8601 formatted string denoting when this message was
    /// received by the server, e.g. `"2022-07-09T11:45:24.888Z"`.
    /// This is the "absolute ordering" for chat messages,
    /// however the time can potentially differ several seconds between the client and server because of latency and clock skew.
    ///
    /// The string is only valid for the duration of the call.
    timestamp: *const c_char,
    timestamp_length: u64,

    /// Null terminated account name of the player that sent the message, including leading ':'.
    ///
    /// The string is only valid for the duration of the call.
    account_name: *const c_char,
    account_name_length: u64,

    /// Null terminated character name of the player that sent the message.
    ///
    /// The string is only valid for the duration of the call.
    character_name: *const c_char,
    character_name_length: u64,

    /// Null terminated string containing the content of the message that was sent.
    ///
    /// The string is only valid for the duration of the call.
    text: *const c_char,
    text_length: u64,
}

impl SquadMessage {
    /// Converts the squad message to its owned counterpart.
    #[inline]
    pub fn to_owned(&self) -> SquadMessageOwned {
        self.into()
    }

    /// Returns the raw message flags.
    #[inline]
    pub fn flags_raw(&self) -> u8 {
        self.flags
    }

    /// Returns the message flags.
    #[inline]
    pub fn flags(&self) -> SquadMessageFlags {
        SquadMessageFlags::from_bits_truncate(self.flags)
    }

    /// Returns the message flags.
    #[inline]
    pub fn is_broadcast(&self) -> bool {
        self.flags().contains(SquadMessageFlags::IS_BROADCAST)
    }

    /// Returns the timestamp as string.
    #[inline]
    pub fn timestamp_str(&self) -> &str {
        unsafe { str_from_cstr_len(self.timestamp, self.timestamp_length) }
    }

    /// Returns the timestamp string as raw pointer.
    #[inline]
    pub fn timestamp_ptr(&self) -> *const c_char {
        self.timestamp
    }

    /// Returns the timestamp string length.
    #[inline]
    pub fn timestamp_len(&self) -> usize {
        self.timestamp_length as _
    }

    /// Returns the timestamp when the message was received.
    ///
    /// This is the "absolute ordering" for chat messages,
    /// however the time can potentially differ several seconds between the client and server because of latency and clock skew.
    #[inline]
    pub fn timestamp(&self) -> Option<DateTime<FixedOffset>> {
        DateTime::parse_from_rfc3339(self.timestamp_str()).ok()
    }

    /// Returns the account name of the player that sent the message.
    #[inline]
    pub fn account_name(&self) -> &str {
        let account_name =
            unsafe { str_from_cstr_len(self.account_name, self.account_name_length) };
        strip_account_prefix(account_name)
    }

    /// Returns the account name as raw pointer.
    #[inline]
    pub fn account_name_ptr(&self) -> *const c_char {
        self.account_name
    }

    /// Returns the account name length.
    #[inline]
    pub fn account_name_len(&self) -> usize {
        self.account_name_length as _
    }

    /// Returns the character name of the player that sent the message.
    #[inline]
    pub fn character_name(&self) -> &str {
        unsafe { str_from_cstr_len(self.character_name, self.character_name_length) }
    }

    /// Returns the character name as raw pointer.
    #[inline]
    pub fn character_name_ptr(&self) -> *const c_char {
        self.character_name
    }

    /// Returns the account name length.
    #[inline]
    pub fn character_name_len(&self) -> usize {
        self.character_name_length as _
    }

    /// Returns the text content of the message.
    #[inline]
    pub fn text(&self) -> &str {
        unsafe { str_from_cstr_len(self.text, self.text_length) }
    }

    /// Returns the text as raw pointer.
    #[inline]
    pub fn text_ptr(&self) -> *const c_char {
        self.text
    }

    /// Returns the account name length.
    #[inline]
    pub fn text_len(&self) -> usize {
        self.text_length as _
    }
}

/// [`SquadMessage`] with owned [`String`] fields.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SquadMessageOwned {
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
    pub flags: SquadMessageFlags,

    /// Timestamp when the message was received.
    ///
    /// This is the "absolute ordering" for chat messages,
    /// however the time can potentially differ several seconds between the client and server because of latency and clock skew.
    pub timestamp: Option<DateTime<FixedOffset>>,

    /// Account name of the player that sent the message.
    pub account_name: String,

    /// Character name of the player that sent the message.
    pub character_name: String,

    /// Content of the message.
    pub text: String,
}

impl From<SquadMessage> for SquadMessageOwned {
    #[inline]
    fn from(msg: SquadMessage) -> Self {
        (&msg).into()
    }
}

impl From<&SquadMessage> for SquadMessageOwned {
    #[inline]
    fn from(msg: &SquadMessage) -> Self {
        Self {
            channel_id: msg.channel_id,
            channel_type: msg.channel_type,
            subgroup: msg.subgroup,
            flags: SquadMessageFlags::from_bits_truncate(msg.flags),
            timestamp: msg.timestamp(),
            account_name: msg.account_name().to_owned(),
            character_name: msg.character_name().to_owned(),
            text: msg.text().to_owned(),
        }
    }
}

bitflags! {
    /// Squad message flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct SquadMessageFlags : u8 {
        /// Message is a broadcast.
        const IS_BROADCAST = 1;
    }
}

/// Type of message channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum ChannelType {
    Party = 0,
    Squad = 1,
    Reserved = 2,
    Invalid = 3,
}
