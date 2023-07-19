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
    Effect = 0,

    /// Content is a marker.
    Marker = 1,
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

    /// Location of the effect.
    pub location: EffectLocation,

    /// Duration of the effect in milliseconds.
    pub duration: u32,

    /// Trackable id for effect end.
    pub tracking_id: u32,

    /// Effect orientation.
    pub orientation: EffectOrientation,
}

impl Effect {
    /// Extracts effect information from an [`StateChange::Effect`] event.
    #[inline]
    pub fn from_event(event: &CombatEvent) -> Self {
        let effect_id = event.skill_id;
        let duration: u32 = unsafe {
            transmute([
                event.affinity.into(),
                event.buff,
                event.result,
                event.is_activation.into(),
            ])
        };
        let tracking_id: u32 = unsafe {
            transmute([
                event.is_buff_remove.into(),
                event.is_ninety,
                event.is_fifty,
                event.is_moving,
            ])
        };
        let orientation: [i16; 3] = unsafe {
            transmute([
                event.is_shields,
                event.is_off_cycle,
                event.pad61,
                event.pad62,
                event.pad63,
                event.pad64,
            ])
        };

        Self {
            effect_id,
            owner: event.src_agent,
            location: EffectLocation::from_event(event),
            duration,
            tracking_id,
            orientation,
        }
    }

    /// Checks whether this is the end of an effect.
    #[inline]
    pub fn is_end(&self) -> bool {
        self.effect_id == 0
    }
}

impl TryFrom<&CombatEvent> for Effect {
    type Error = ();

    #[inline]
    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::Effect => Ok(Self::from_event(event)),
            _ => Err(()),
        }
    }
}

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

/// Location of an effect.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EffectLocation {
    Agent(u64),
    Position(Position),
}

impl EffectLocation {
    /// Extracts an effect location from an effect [`CombatEvent`].
    #[inline]
    pub fn from_event(event: &CombatEvent) -> Self {
        if event.dst_agent != 0 {
            Self::Agent(event.dst_agent)
        } else {
            let pos: [f32; 3] =
                unsafe { transmute((event.value, event.buff_dmg, event.overstack_value)) };
            Self::Position(pos.into())
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

/// Orientation of an effect.
///
/// Value ranges from `-31415` (-PI) to `+31415` (+PI) or [`i16::MIN`]/[`i16::MAX`] if out of those bounds.
pub type EffectOrientation = [i16; 3];
