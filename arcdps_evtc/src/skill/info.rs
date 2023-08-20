use crate::{extract::Extract, CombatEvent, StateChange, TryExtract};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Skill information from a [`CombatEvent`] with [`StateChange::SkillInfo`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SkillInfo {
    pub recharge: f32,
    pub range0: f32,
    pub range1: f32,
    pub tooltip_time: f32,
}

impl Extract for SkillInfo {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        let [recharge, range0, range1, tooltip_time]: [f32; 4] =
            transmute((event.time, event.src_agent));
        Self {
            recharge,
            range0,
            range1,
            tooltip_time,
        }
    }
}

impl TryExtract for SkillInfo {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::SkillInfo
    }
}
