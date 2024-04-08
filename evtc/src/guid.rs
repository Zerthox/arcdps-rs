use crate::{
    extract::{transmute_field, Extract},
    Event, StateChange, TryExtract,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Content GUID information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContentGUID {
    /// Id of the content (volatile, depends on game build).
    pub content_id: u32,

    /// Persistent content GUID.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_hex"))]
    pub guid: u128,

    /// Content local.
    pub content_local: Option<ContentLocal>,
}

impl ContentGUID {
    /// Formats the contained GUID as [`String`].
    #[inline]
    pub fn guid_string(&self) -> String {
        format!("{:0>32X}", self.guid)
    }

    /// Whether the GUID is an effect.
    #[inline]
    pub fn is_effect(&self) -> bool {
        matches!(self.content_local, Some(ContentLocal::Effect))
    }

    /// Whether the GUID is a marker.
    #[inline]
    pub fn is_marker(&self) -> bool {
        matches!(self.content_local, Some(ContentLocal::Marker))
    }
}

impl Extract for ContentGUID {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        // TODO: why big endian here?
        Self {
            content_id: event.skill_id,
            guid: u128::from_be(transmute_field!(event.src_agent as u128)),
            content_local: event.overstack_value.try_into().ok(),
        }
    }
}

impl TryExtract for ContentGUID {
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
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
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

        let info = ContentGUID::try_extract(&event).expect("failed to extract");
        assert_eq!(info.guid, 0x1B56F702912BE7428182CA57036AEE99);
        assert_eq!(info.guid_string(), "1B56F702912BE7428182CA57036AEE99");
    }
}
