use crate::{
    extract::{transmute_field, Extract},
    Event, StateChange, TryExtract,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Effect information from an [`Event`] with [`StateChange::IdToGUID`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        // TODO: why big endian here?
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn guid_extraction() {
        let event = Event {
            is_statechange: StateChange::IdToGUID.into(),
            src_agent: 4820869827943421467,
            dst_agent: 11091919494850445953,
            skill_id: 446,
            overstack_value: 0,
            ..unsafe { mem::zeroed() }
        };
        assert_eq!(event.src_agent, 0x42E72B9102F7561B);
        assert_eq!(event.dst_agent, 0x99EE6A0357CA8281);

        let effect = EffectGUID::try_extract(&event).expect("failed to extract");
        assert_eq!(effect.guid, 0x1B56F702912BE7428182CA57036AEE99);
        assert_eq!(effect.guid_string(), "1B56F702912BE7428182CA57036AEE99");
    }
}
