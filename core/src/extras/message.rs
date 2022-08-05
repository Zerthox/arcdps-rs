use std::os::raw::c_char;

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
    pub unused1: u16,
    pub timestamp: String,
    pub account_name: String,
    pub character_name: String,
    pub text: String,
}

impl From<RawChatMessageInfo> for ChatMessageInfoOwned {
    fn from(raw: RawChatMessageInfo) -> Self {
        let timestamp = unsafe { get_str_from_ptr_and_len(msg.timestamp, msg.timestamp_length) };
        let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap();

        let account_name =
            unsafe { get_str_from_ptr_and_len(msg.account_name, msg.account_name_length) };
        let character_name =
            unsafe { get_str_from_ptr_and_len(msg.character_name, msg.character_name_length) };
        let text = unsafe { get_str_from_ptr_and_len(msg.text, msg.text_length) };
        let is_broadcast = (msg.is_broadcast & 0x01) != 0;

        Self {
            channel_id: msg.channel_id,
            channel_type: msg.channel_type,
            subgroup: msg.subgroup,
            is_broadcast,
            timestamp,
            account_name: account_name.trim_start_matches(':'),
            character_name,
            text,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessageInfo<'a> {
    pub channel_id: u32,
    pub channel_type: ChannelType,
    pub subgroup: u8,
    pub unused1: u16,
    pub timestamp: &'a str,
    pub account_name: &'a str,
    pub character_name: &'a str,
    pub text: &'a str,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawChatMessageInfo {
    pub channel_id: u32,
    pub channel_type: ChannelType,
    pub subgroup: u8,
    pub unused1: u16,
    pub timestamp: *const c_char,
    pub timestamp_length: u64,
    pub account_name: *const c_char,
    pub account_name_length: u64,
    pub character_name: *const c_char,
    pub character_name_length: u64,
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

/// Converts a pointer and length into a &str with a lifetime. The pointer must not be null
#[inline(always)]
unsafe fn get_str_from_ptr_and_len(src: *const u8, len: u64) -> &'static str {
    let buff = std::slice::from_raw_parts(src, len as usize);
    std::str::from_utf8_unchecked(buff)
}