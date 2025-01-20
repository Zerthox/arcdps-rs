//! Message information provided by Unofficial Extras.

use crate::{strip_account_prefix, util::str_from_cstr_len};
use chrono::{DateTime, FixedOffset};
use std::os::raw::c_char;

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
/// # let message: SquadMessage = todo!();
/// let owned = message.to_owned();
/// let owned: SquadMessageOwned = message.into();
/// ```
#[derive(Debug, Clone)]
pub struct SquadMessage<'a> {
    /// A unique identifier for the channel this chat message was sent over.
    ///
    /// Can be used to for example differentiate between squad messages sent to different squads.
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

impl SquadMessage<'_> {
    /// Converts the message to its owned counterpart.
    #[inline]
    pub fn to_owned(self) -> SquadMessageOwned {
        self.into()
    }
}

impl<'a> From<&'a RawSquadMessage> for SquadMessage<'a> {
    fn from(raw: &RawSquadMessage) -> Self {
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

impl From<SquadMessage<'_>> for SquadMessageOwned {
    #[inline]
    fn from(msg: SquadMessage<'_>) -> Self {
        Self {
            channel_id: msg.channel_id,
            channel_type: msg.channel_type,
            subgroup: msg.subgroup,
            is_broadcast: msg.is_broadcast,
            timestamp: msg.timestamp,
            account_name: msg.account_name.to_string(),
            character_name: msg.character_name.to_string(),
            text: msg.text.to_string(),
        }
    }
}

/// Raw chat message information.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawSquadMessage {
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

/// An NPC chat message.
///
/// Strings are available for the duration of the call.
/// If you need it for longer than that, consider converting it to [`NpcMessageOwned`].
///
/// ```no_run
/// # use arcdps::extras::{NpcMessage, NpcMessageOwned};
/// # let message: NpcMessage = todo!();
/// let owned = message.to_owned();
/// let owned: NpcMessageOwned = message.into();
/// ```
#[derive(Debug, Clone)]
pub struct NpcMessage<'a> {
    /// Character name of the NPC that sent the message.
    pub character_name: &'a str,

    /// Content of the message.
    pub text: &'a str,
}

impl NpcMessage<'_> {
    /// Converts the message to its owned counterpart.
    #[inline]
    pub fn to_owned(self) -> NpcMessageOwned {
        self.into()
    }
}

impl<'a> From<&'a RawNpcMessage> for NpcMessage<'a> {
    #[inline]
    fn from(raw: &RawNpcMessage) -> Self {
        let character_name =
            unsafe { str_from_cstr_len(raw.character_name, raw.character_name_length) };
        let text = unsafe { str_from_cstr_len(raw.text, raw.text_length) };

        Self {
            character_name,
            text,
        }
    }
}

/// [`NpcMessage`] with owned [`String`] fields.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NpcMessageOwned {
    /// Character name of the NPC that sent the message.
    pub character_name: String,

    /// Content of the message.
    pub text: String,
}

impl From<NpcMessage<'_>> for NpcMessageOwned {
    #[inline]
    fn from(msg: NpcMessage<'_>) -> Self {
        Self {
            character_name: msg.character_name.to_string(),
            text: msg.text.to_string(),
        }
    }
}

/// Raw NPC chat message information.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawNpcMessage {
    /// Null terminated character name of the NPC that sent the message.
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

/// A chat message.
#[derive(Clone, Copy)]
#[repr(C)]
pub union RawMessage {
    squad: *const RawSquadMessage,
    npc: *const RawNpcMessage,
}

/// A chat message.
#[derive(Debug, Clone)]
pub enum Message<'a> {
    Squad(SquadMessage<'a>),
    Npc(NpcMessage<'a>),
}

impl Message<'_> {
    /// Creates a new message from [`RawMessageType`] and [`RawMessage`].
    #[inline]
    pub unsafe fn new(message_type: RawMessageType, message: RawMessage) -> Self {
        match message_type {
            RawMessageType::Squad => Self::Squad(
                message
                    .squad
                    .as_ref()
                    .expect("invalid unofficial extras squad message info")
                    .into(),
            ),
            RawMessageType::Npc => Self::Npc(
                message
                    .npc
                    .as_ref()
                    .expect("invalid unofficial extras npc message info")
                    .into(),
            ),
        }
    }
}

/// Type of message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(C)]
pub enum RawMessageType {
    /// Party or squad chat message.
    Squad = 0,

    /// NPC message (selectable in ingame-chat as "NPC").
    Npc = 1,
}
