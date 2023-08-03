use crate::{CombatEvent, Extract, StateChange};

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

impl TryFrom<&CombatEvent> for SkillTiming {
    type Error = ();

    #[inline]
    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::SkillTiming => Ok(unsafe { Self::extract(event) }),
            _ => Err(()),
        }
    }
}
