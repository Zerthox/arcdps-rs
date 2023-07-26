use crate::{CombatEvent, StateChange};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Effect information from a [`CombatEvent`] with [`StateChange::IdToGUID`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EffectGUID {
    pub effect_id: u32,
    pub guid: u128,
    pub content_local: Option<ContentLocal>,
}

impl EffectGUID {
    /// Extracts effect GUID information from a [`StateChange::IdToGUID`] event.
    #[inline]
    pub fn from_event(event: &CombatEvent) -> Self {
        Self {
            effect_id: event.skill_id,
            guid: u128::from_be_bytes(unsafe { transmute([event.src_agent, event.dst_agent]) }),
            content_local: event.overstack_value.try_into().ok(),
        }
    }
}

impl TryFrom<&CombatEvent> for EffectGUID {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::IdToGUID => Ok(Self::from_event(event)),
            _ => Err(()),
        }
    }
}

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
