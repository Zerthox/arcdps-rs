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

impl TryFrom<&CombatEvent> for SkillInfo {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::SkillInfo => {
                let [recharge, range0, range1, tooltip_time]: [f32; 4] =
                    unsafe { transmute((event.time, event.src_agent)) };
                Ok(Self {
                    recharge,
                    range0,
                    range1,
                    tooltip_time,
                })
            }

            _ => Err(()),
        }
    }
}

/// Skill timing from a [`CombatEvent`] with [`StateChange::SkillTiming`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SkillTiming {
    pub action: u64,
    pub millisecond: u64,
}

impl TryFrom<&CombatEvent> for SkillTiming {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::SkillTiming => Ok(Self {
                action: event.src_agent,
                millisecond: event.dst_agent,
            }),

            _ => Err(()),
        }
    }
}
