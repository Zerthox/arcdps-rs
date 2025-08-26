use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Agent team change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TeamChangeEvent {
    /// Time of registering the team change.
    pub time: u64,

    /// Agent that had their team changed.
    pub agent: AgentId,

    /// New team id.
    pub team: u64,

    /// Previous team id.
    pub previous_team: i32,
}

impl Extract for TeamChangeEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            team: event.dst_agent,
            previous_team: event.value,
        }
    }
}

impl TryExtract for TeamChangeEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::TeamChange
    }
}
