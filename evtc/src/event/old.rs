#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An event for old EVTC revision (`header[12] == 0`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct EventOld {
    pub time: u64,
    pub src_agent: u64,
    pub dst_agent: u64,
    pub value: i32,
    pub buff_dmg: i32,
    pub overstack_value: u16,
    pub skillid: u16,
    pub src_instid: u16,
    pub dst_instid: u16,
    pub src_master_instid: u16,
    pub iss_offset: u8,
    pub iss_offset_target: u8,
    pub iss_bd_offset: u8,
    pub iss_bd_offset_target: u8,
    pub iss_alt_offset: u8,
    pub iss_alt_offset_target: u8,
    pub skar: u8,
    pub skar_alt: u8,
    pub skar_use_alt: u8,
    pub iff: u8,
    pub buff: u8,
    pub result: u8,
    pub is_activation: u8,
    pub is_buffremove: u8,
    pub is_ninety: u8,
    pub is_fifty: u8,
    pub is_moving: u8,
    pub is_statechange: u8,
    pub is_flanking: u8,
    pub is_shields: u8,
    pub is_offcycle: u8,
    pub pad64: u8,
}
