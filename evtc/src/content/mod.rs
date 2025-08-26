//! Bindings & utilities for GUIDs appearing in events.

mod guid;

#[cfg(feature = "serde")]
pub mod serde_guid;

use crate::{
    extract::{transmute_field, Extract},
    Event, StateChange, TryExtract,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

pub use self::guid::*;

/// Content information.
///
/// The contained GUID is interpreted as a Windows [`GUID`].
/// See https://learn.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid for more information.
///
/// Some GW2 community projects misinterpret the memory layout of the GUID as bytes rather than a Windows [`GUID`].
/// When comparing or interfacing with such projects, you can use [`GuidExt::misinterpret`] on the [`GUID`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContentInfo {
    /// Id of the content (volatile, depends on game build).
    pub content_id: u32,

    /// Persistent content GUID.
    #[cfg_attr(feature = "serde", serde(with = "serde_guid"))]
    pub guid: GUID,

    /// Content type.
    pub content_type: ContentType,
}

impl ContentInfo {
    /// Formats the contained GUID as [`String`].
    #[inline]
    pub fn guid_string(&self) -> String {
        self.guid.format_simple()
    }

    /// Whether the content is an effect.
    #[inline]
    pub fn is_effect(&self) -> bool {
        matches!(self.content_type, ContentType::Effect { .. })
    }

    /// Whether the content is a marker.
    #[inline]
    pub fn is_marker(&self) -> bool {
        matches!(self.content_type, ContentType::Marker { .. })
    }

    /// Whether the content is a skill.
    #[inline]
    pub fn is_skill(&self) -> bool {
        matches!(self.content_type, ContentType::Skill)
    }

    /// Whether the content is a species.
    #[inline]
    pub fn is_species(&self) -> bool {
        matches!(self.content_type, ContentType::Species)
    }
}

impl Extract for ContentInfo {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            content_id: event.skill_id,
            guid: transmute_field!(event.src_agent as GUID),
            content_type: event.extract(),
        }
    }
}

impl TryExtract for ContentInfo {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::IdToGUID
    }
}

/// Content type for [`ContentInfo`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ContentType {
    /// Effect.
    Effect {
        /// Effect type.
        effect_type: u16,

        /// Default effect duration, if available.
        default_duration: f32,
    },

    /// Marker.
    Marker {
        /// Is in commander tag defs.
        is_commander_tag: bool,
    },

    /// Skill.
    ///
    /// See skill & buff info events for extra information.
    Skill,

    /// Species (only characters, not gadgets).
    Species,

    /// Unknown content type.
    Unknown(u32),
}

impl Extract for ContentType {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        match event.overstack_value.try_into() {
            Ok(ContentLocal::Effect) => Self::Effect {
                effect_type: event.src_instance_id,
                default_duration: transmute_field!(event.buff_dmg as f32),
            },
            Ok(ContentLocal::Marker) => Self::Marker {
                is_commander_tag: event.src_instance_id != 0,
            },
            Ok(ContentLocal::Skill) => Self::Skill,
            Ok(ContentLocal::Species) => Self::Species,
            Err(err) => Self::Unknown(err.number),
        }
    }
}

impl TryExtract for ContentType {
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
    ///
    /// `src_instance_id` contains the effect type.
    /// `buff_dmg` contains the default duration as [`f32`], if available.
    Effect = 0,

    /// Content is a marker.
    Marker = 1,

    /// Content is a skill.
    ///
    /// See skill & buff info events for extra information.
    Skill = 2,

    /// Content is a species (only characters, not gadgets).
    Species = 3,
}
