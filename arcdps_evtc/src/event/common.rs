use crate::{Affinity, CombatEvent};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Information common to most events.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommonEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that caused the event.
    pub src_agent: u64,

    /// Agent the event happened to.
    pub dst_agent: u64,

    /// Instance id of source agent as appears in game at time of event.
    pub src_instance_id: u16,

    /// Instance id of destination agent as appears in game at time of event.
    pub dst_instance_id: u16,

    /// If `src_agent` has a master (e.g. is minion), will be equal to instance id of master, zero otherwise.
    pub src_master_instance_id: u16,

    /// If `dst_agent` has a master (e.g. is minion), will be equal to instance id of master, zero otherwise.
    pub dst_master_instance_id: u16,

    /// Skill id of the relevant skill (can be zero).
    pub skill_id: u32,

    /// Current affinity of `src_agent` and `dst_agent`.
    ///
    /// *Arc calls this "iff" for if friend/foe.*
    pub affinity: Affinity,

    /// Whether `src_agent` is above 90% Health.
    pub is_ninety: u8,

    /// Whether `dst_agent` is below 50% Health.
    pub is_fifty: u8,

    /// Whether `src_agent` is moving at time of event.
    pub is_moving: u8,

    /// Whether `src_agent` is flanking at time of event.
    ///
    /// The value lies in a range of `1` to `135` degrees where `135` is rear.
    pub is_flanking: u8,
}

impl From<&CombatEvent> for CommonEvent {
    fn from(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            src_agent: event.src_agent,
            dst_agent: event.dst_agent,
            src_instance_id: event.src_instance_id,
            dst_instance_id: event.dst_instance_id,
            src_master_instance_id: event.src_master_instance_id,
            dst_master_instance_id: event.dst_master_instance_id,
            skill_id: event.skill_id,
            affinity: event.affinity,
            is_ninety: event.is_ninety,
            is_fifty: event.is_fifty,
            is_moving: event.is_moving,
            is_flanking: event.is_flanking,
        }
    }
}
