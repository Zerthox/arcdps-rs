use super::EffectLocation;
use crate::{CombatEvent, Position, StateChange};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Effect information from a [`CombatEvent`] with [`StateChange::EffectOld`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EffectOld {
    /// Id of the effect.
    ///
    /// Use to map to a GUID using [`StateChange::IdToGUID`] events.
    pub effect_id: u32,

    /// Owner of the effect.
    pub owner: u64,

    /// Location of the effect.
    pub location: EffectLocation,

    /// Orientation of the effect as 3 dimensional vector.
    pub orientation: Position,

    /// Duration of the effect in time or as tracking id.
    pub duration: EffectDuration,
}

impl EffectOld {
    /// Extracts effect information from a [`StateChange::EffectOld`] event.
    #[inline]
    pub fn from_event(event: &CombatEvent) -> Self {
        let effect_id = event.skill_id;
        let [x, y]: [f32; 2] = unsafe {
            transmute([
                event.affinity.into(),
                event.buff,
                event.result,
                event.is_activation.into(),
                event.is_buff_remove.into(),
                event.is_ninety,
                event.is_fifty,
                event.is_moving,
            ])
        };
        let z: f32 = unsafe { transmute([event.pad61, event.pad62, event.pad63, event.pad64]) };
        let duration: u16 = unsafe { transmute([event.is_shields, event.is_off_cycle]) };

        Self {
            effect_id,
            owner: event.src_agent,
            location: EffectLocation::from_event(event),
            orientation: [x, y, z].into(),
            duration: if event.is_flanking != 0 || effect_id == 0 {
                EffectDuration::TrackingId(duration)
            } else {
                EffectDuration::Time(duration)
            },
        }
    }

    /// Checks whether this is the end of an effect.
    #[inline]
    pub fn is_end(&self) -> bool {
        self.effect_id == 0
    }
}

impl TryFrom<&CombatEvent> for EffectOld {
    type Error = ();

    #[inline]
    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::EffectOld => Ok(Self::from_event(event)),
            _ => Err(()),
        }
    }
}

/// Duration of an effect in time or as a tracking id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EffectDuration {
    /// Duration as time in milliseconds.
    Time(u16),

    /// Duration as tracking id.
    TrackingId(u16),
}
