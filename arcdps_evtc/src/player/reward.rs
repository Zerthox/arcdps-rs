use crate::{extract::Extract, AgentId, CombatEvent, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Reward chest received.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RewardEvent {
    /// Time of registering the reward.
    pub time: u64,

    /// Agent that is self.
    pub agent: AgentId,

    /// Reward id.
    pub reward: u64,
}

impl Extract for RewardEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            reward: event.dst_agent,
        }
    }
}

impl TryExtract for RewardEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::Reward
    }
}
