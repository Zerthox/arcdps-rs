use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Simple event regarding an agent.
///
/// The meaning depends on the context.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentStatusEvent {
    /// Time of registering the status change.
    pub time: u64,

    /// Agent that the status change happened to.
    pub agent: AgentId,
}

impl Extract for AgentStatusEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
        }
    }
}

impl TryExtract for AgentStatusEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        matches!(
            event.get_statechange(),
            StateChange::ExitCombat
                | StateChange::ChangeUp
                | StateChange::ChangeDead
                | StateChange::ChangeDown
                | StateChange::Spawn
                | StateChange::Despawn
                | StateChange::PointOfView
        )
    }
}

/// Agent down contribution event (retired).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DownContributionEvent {
    /// Time of registering the downed state.
    pub time: u64,

    /// Agent that entered downed state.
    pub agent: AgentId,

    /// Time since last 90% HP in milliseconds.
    pub time_frame: u64,
}

impl Extract for DownContributionEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            time_frame: event.dst_agent,
        }
    }
}

impl TryExtract for DownContributionEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Last90BeforeDown
    }
}
