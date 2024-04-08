#![allow(deprecated)]

use crate::{
    extract::{transmute_field, Extract},
    guid::ContentLocal,
    Event, StateChange, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Effect information from an [`Event`] with [`StateChange::IdToGUID`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deprecated(since = "0.9.0", note = "replaced by content guid event")]
pub struct EffectGUID {
    /// Id of the effect.
    pub effect_id: u32,

    /// Persistent content GUID.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_hex"))]
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
    unsafe fn extract(event: &Event) -> Self {
        Self {
            effect_id: event.skill_id,
            guid: u128::from_be(transmute_field!(event.src_agent as u128)),
            content_local: event.overstack_value.try_into().ok(),
        }
    }
}

impl TryExtract for EffectGUID {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::IdToGUID
    }
}
