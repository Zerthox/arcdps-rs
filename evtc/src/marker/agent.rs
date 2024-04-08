use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Agent has a marker.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentMarkerEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that has the marker.
    pub agent: AgentId,

    /// Marker id.
    ///
    /// Id is volatile, depends on game build.
    pub marker: i32,

    /// Non-zero if commander.
    pub commander: u8,
}

impl AgentMarkerEvent {
    /// Whether the marker was removed.
    #[inline]
    pub fn is_remove(&self) -> bool {
        self.marker == 0
    }

    /// Whether the marker is a commander tag.
    #[inline]
    pub fn is_commander(&self) -> bool {
        self.commander != 0
    }
}

impl Extract for AgentMarkerEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            marker: event.value,
            commander: event.buff,
        }
    }
}

impl TryExtract for AgentMarkerEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Marker
    }
}
