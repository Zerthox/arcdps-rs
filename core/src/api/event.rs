use super::{Activation, BuffRemove, StateChange, Team};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ArcDPS Combat event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CombatEvent {
    pub time: u64,
    pub src_agent: usize,
    pub dst_agent: usize,
    pub value: i32,
    pub buff_dmg: i32,
    pub overstack_value: u32,
    pub skill_id: u32,
    pub src_instance_id: u16,
    pub dst_instance_id: u16,
    pub src_master_instance_id: u16,
    pub dst_master_instance_id: u16,
    pub iff: Team,
    pub buff: u8,
    pub result: u8,
    pub is_activation: Activation,
    pub is_buff_remove: BuffRemove,
    pub is_ninety: u8,
    pub is_fifty: u8,
    pub is_moving: u8,
    pub is_statechange: StateChange,
    pub is_flanking: u8,
    pub is_shields: u8,
    pub is_off_cycle: u8,
    pub pad61: u8,
    pub pad62: u8,
    pub pad63: u8,
    pub pad64: u8,
}
