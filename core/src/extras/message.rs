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
