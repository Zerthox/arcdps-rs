use crate::{CombatEvent, StateChange};
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

impl SkillInfo {
    /// Extracts skill information from a [`StateChange::SkillInfo`] event.
    ///
    /// # Safety
    /// This operation is safe when the [`CombatEvent`] is a valid skill info event.
    #[inline]
    pub unsafe fn from_event(event: &CombatEvent) -> Self {
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

impl TryFrom<&CombatEvent> for SkillInfo {
    type Error = ();

    #[inline]
    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::SkillInfo => Ok(unsafe { Self::from_event(event) }),
            _ => Err(()),
        }
    }
}
