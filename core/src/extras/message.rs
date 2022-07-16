use std::os::raw::c_char;

#[derive(Debug, Clone)]
pub struct ChatMessageInfo {
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
#[repr(u8)]
pub enum ChannelType {
    Party = 0,
    Squad = 1,
    Reserved = 2,
    Invalid = 3,
}
