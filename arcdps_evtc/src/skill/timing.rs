use crate::{extract::Extract, CombatEvent, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Skill timing from a [`CombatEvent`] with [`StateChange::SkillTiming`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SkillTiming {
    pub action: u64,
    pub millisecond: u64,
}

impl Extract for SkillTiming {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            action: event.src_agent,
            millisecond: event.dst_agent,
        }
    }
}

impl TryExtract for SkillTiming {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::SkillTiming
    }
}
