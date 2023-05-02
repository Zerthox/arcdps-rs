use crate::{CombatEvent, Position, StateChange};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Content local for [`StateChange::IdToGUID`] events.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u32)]
pub enum ContentLocal {
    /// Content is an effect.
    Effect,

    /// Content is a marker.
    Marker,
}

/// Effect information from a [`CombatEvent`] with [`StateChange::Effect`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Effect {
    /// Id of the effect.
    ///
    /// Use to map to a GUID using [`StateChange::IdToGUID`] events.
    pub effect_id: u32,

    /// Owner of the effect.
    pub owner: u64,

    /// Agent the effect is located at, if the case.
    pub agent_location: u64,

    /// Location of the effect, if not located at agent.
    pub location: Position,

    /// Orientation of the effect, if not located at agent.
    pub orientation: Position,

    /// Duration of the effect in time or as tracking id.
    pub duration: EffectDuration,
}

impl TryFrom<&CombatEvent> for Effect {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::Effect => {
                let effect_id = event.skill_id;
                let pos: [f32; 3] =
                    unsafe { transmute((event.value, event.buff_dmg, event.overstack_value)) };
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
                let z: f32 =
                    unsafe { transmute([event.pad61, event.pad62, event.pad63, event.pad64]) };
                let duration: u16 = unsafe { transmute([event.is_shields, event.is_off_cycle]) };

                Ok(Self {
                    effect_id,
                    owner: event.src_agent,
                    agent_location: event.dst_agent,
                    location: pos.into(),
                    orientation: [x, y, z].into(),
                    duration: if event.is_flanking != 0 || effect_id == 0 {
                        EffectDuration::TrackingId(duration)
                    } else {
                        EffectDuration::Time(duration)
                    },
                })
            }

            _ => Err(()),
        }
    }
}

/// Duration of an effect in time or as a tracking id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EffectDuration {
    Time(u16),
    TrackingId(u16),
}
