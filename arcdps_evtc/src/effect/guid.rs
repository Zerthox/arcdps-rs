use crate::{extract::Extract, CombatEvent, StateChange, TryExtract};
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
    /// Id of the effect.
    pub effect_id: u32,

    /// Persistent content GUID.
    pub guid: u128,

    /// Content local.
    pub content_local: Option<ContentLocal>,
}

impl EffectGUID {
    /// Formats the contained GUID as [`String`].
    #[inline]
    pub fn guid_string(&self) -> String {
        format!("{:0>32X}", self.guid)
    }
}

impl Extract for EffectGUID {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            effect_id: event.skill_id,
            guid: u128::from_be_bytes(transmute([event.src_agent, event.dst_agent])),
            content_local: event.overstack_value.try_into().ok(),
        }
    }
}

impl TryExtract for EffectGUID {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::IdToGUID
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
