use crate::{extract::Extract, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Skill timing from an [`Event`] with [`StateChange::SkillTiming`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SkillTiming {
    pub action: u64,
    pub millisecond: u64,
}

impl Extract for SkillTiming {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            action: event.src_agent,
            millisecond: event.dst_agent,
        }
    }
}

impl TryExtract for SkillTiming {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::SkillTiming
    }
}
