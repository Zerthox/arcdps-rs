use crate::{
    extract::{transmute_field, Extract},
    AgentId, Event, StateChange, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Active buff stack change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StackActiveEvent {
    /// Time of registering the active buff stack change.
    pub time: u64,

    /// Agent that had their active buff stack changed.
    pub agent: AgentId,

    /// Stack id of new active stack.
    pub stack_id: u64,

    /// Current buff duration.
    pub duration: i32,
}

impl Extract for StackActiveEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            stack_id: event.dst_agent,
            duration: event.value,
        }
    }
}

impl TryExtract for StackActiveEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::StackActive
    }
}

/// Buff stack reset.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StackResetEvent {
    /// Time of registering the stack reset.
    pub time: u64,

    /// Agent that the stack reset happened to.
    pub agent: AgentId,

    /// New duration to reset to.
    pub duration: i32,

    /// Stack id.
    pub stack_id: u32,
}

impl Extract for StackResetEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            duration: event.value,
            stack_id: transmute_field!(event.pad61 as u32),
        }
    }
}

impl TryExtract for StackResetEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::StackReset
    }
}
