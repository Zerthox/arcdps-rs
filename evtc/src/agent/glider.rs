use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Agent gliding state change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GliderEvent {
    /// Time of registering the gliding state.
    pub time: u64,

    /// Agent that changed gliding state.
    pub agent: AgentId,

    pub deployed: bool,
}

impl Extract for GliderEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            deployed: event.value != 0,
        }
    }
}

impl TryExtract for GliderEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Glider
    }
}
