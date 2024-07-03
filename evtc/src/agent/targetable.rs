use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Agent targetable state change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TargetableEvent {
    /// Time of registering the targetable state change.
    pub time: u64,

    /// Agent that had their targetable state changed.
    pub agent: AgentId,

    /// New targetable state.
    pub targetable: bool,
}

impl Extract for TargetableEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            targetable: event.value != 0,
        }
    }
}

impl TryExtract for TargetableEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Targetable
    }
}
