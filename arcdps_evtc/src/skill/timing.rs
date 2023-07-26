use crate::{CombatEvent, StateChange};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Skill timing from a [`CombatEvent`] with [`StateChange::SkillTiming`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SkillTiming {
    pub action: u64,
    pub millisecond: u64,
}

impl SkillTiming {
    /// Extracts skill timing from a [`StateChange::SkillTiming`] event.
    #[inline]
    pub fn from_event(event: &CombatEvent) -> Self {
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
            StateChange::SkillTiming => Ok(Self::from_event(event)),
            _ => Err(()),
        }
    }
}
