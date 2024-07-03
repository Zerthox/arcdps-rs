use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Agent is now an attack target.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttackTargetEvent {
    /// Time of registering the attack target.
    pub time: u64,

    /// Agent that is an attack target.
    pub agent: AgentId,

    /// Parent gadget agent.
    pub parent: AgentId,

    /// Current targetable state.
    pub targetable: bool,
}

impl Extract for AttackTargetEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            parent: AgentId::from_dst(event),
            targetable: event.value != 0,
        }
    }
}

impl TryExtract for AttackTargetEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::AttackTarget
    }
}
