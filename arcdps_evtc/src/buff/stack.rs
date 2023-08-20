use crate::{extract::Extract, AgentId, CombatEvent, StateChange, TryExtract};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Active buff stack changed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StackActiveEvent {
    pub time: u64,
    pub agent: AgentId,
    pub stack_id: u64,
}

impl Extract for StackActiveEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            stack_id: event.dst_agent,
        }
    }
}

impl TryExtract for StackActiveEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::StackActive
    }
}

/// Buff stack reset.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StackResetEvent {
    pub time: u64,
    pub agent: AgentId,
    pub duration: i32,
    pub stack_id: u32,
}

impl Extract for StackResetEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            duration: event.value,
            stack_id: transmute([event.pad61, event.pad62, event.pad63, event.pad64]),
        }
    }
}

impl TryExtract for StackResetEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::StackReset
    }
}
