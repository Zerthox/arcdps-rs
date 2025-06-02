use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Stunbreak event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StunbreakEvent {
    /// Time of registering the stunbreak.
    pub time: u64,

    /// Agent stopping the disable.
    pub agent: AgentId,

    /// Duration remaining.
    pub duration_remaining: i32,
}

impl Extract for StunbreakEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            duration_remaining: event.value,
        }
    }
}

impl TryExtract for StunbreakEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Stunbreak
    }
}
