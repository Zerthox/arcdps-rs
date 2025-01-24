use crate::{
    extract::{transmute_field, Extract},
    Event, StateChange, TryExtract,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::mem;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

pub use windows::core::GUID;

/// Content GUID information.
///
/// The contained GUID is interpreted as a Windows [`GUID`].
/// See https://learn.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid for more information.
///
/// Some GW2 community projects misinterpret the memory layout of the GUID as bytes rather than a Windows [`GUID`].
/// When comparing or interfacing with such projects, you can use [`GuidExt::misinterpret`] on the [`GUID`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContentGUID {
    /// Id of the content (volatile, depends on game build).
    pub content_id: u32,

    /// Persistent content GUID.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_guid"))]
    pub guid: GUID,

    /// Content local.
    pub content_local: Option<ContentLocal>,
}

impl ContentGUID {
    /// Formats the contained GUID as [`String`].
    #[inline]
    pub fn guid_string(&self) -> String {
        self.guid.format_simple()
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
        Self {
            content_id: event.skill_id,
            guid: transmute_field!(event.src_agent as GUID),
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

/// Extensions for [`GUID`].
pub trait GuidExt {
    /// Formats the GUID as a simple hex string.
    fn format_simple(&self) -> String;

    /// Formats the GUID as a hyphenated hex string.
    fn format_hyphenated(&self) -> String;

    /// Returns the contained GUID **misinterpreted** as raw bytes.
    ///
    /// Some GW2 community projects misinterpret the memory layout of the GUID as bytes rather than a Windows [`GUID`].
    /// This is helpful when comparing or interfacing with such projects.
    ///
    /// # Safety
    /// The returned bytes represent the memory of the underlying Windows [`GUID`] struct.
    /// They do not represent the actual GUID.
    /// Constructing a GUID with them will result in a different GUID than the original.
    ///
    /// To get the correct bytes you can convert the GUID to a [`u128`] and then to bytes.
    unsafe fn misinterpret(&self) -> [u8; 16];
}

impl GuidExt for GUID {
    #[inline]
    fn format_simple(&self) -> String {
        format!("{:0>32X}", self.to_u128())
    }

    #[inline]
    fn format_hyphenated(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    unsafe fn misinterpret(&self) -> [u8; 16] {
        mem::transmute::<GUID, [u8; 16]>(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn extract() {
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
        assert_eq!(
            info.guid,
            GUID::from_u128(0x02F7561B_2B91_42E7_8182_CA57036AEE99)
        );
    }

    #[test]
    fn guid_format() {
        let guid = GUID::from_u128(0x02F7561B_2B91_42E7_8182_CA57036AEE99);

        assert_eq!(guid.format_simple(), "02F7561B2B9142E78182CA57036AEE99");
        assert_eq!(
            guid.format_hyphenated(),
            "02F7561B-2B91-42E7-8182-CA57036AEE99"
        );
    }
}
