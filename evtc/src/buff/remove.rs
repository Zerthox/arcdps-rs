use crate::{
    event::CommonEvent,
    extract::{transmute_field, Extract},
    Event, EventCategory, TryExtract,
};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Buff remove event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffRemoveEvent {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Kind of buff remove.
    pub remove: BuffRemove,

    /// Buff.
    // TODO: meaning?
    pub buff: u8,

    /// Removed buff(s) as duration.
    pub removed_duration: i32,

    /// Removed buff(s) as intensity.
    ///
    /// **Warning:** may overflow on [`BuffRemove::All`].
    pub removed_intensity: i32,

    /// Number of stacks removed.
    ///
    /// Only present in [`BuffRemove::All`] events.
    pub stacks_removed: Option<u8>,

    /// Stack (instance) id of removed buff.
    ///
    /// Only present in [`BuffRemove::Single`] events.
    pub stack_id: Option<u32>,
}

impl Extract for BuffRemoveEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let kind = event.get_buffremove();
        Self {
            common: event.into(),
            remove: kind,
            buff: event.buff,
            removed_duration: event.value,
            removed_intensity: event.buff_dmg,
            stacks_removed: if kind == BuffRemove::All {
                Some(event.result)
            } else {
                None
            },
            stack_id: if kind == BuffRemove::Single {
                Some(transmute_field!(event.pad61 as u32))
            } else {
                None
            },
        }
    }
}

impl TryExtract for BuffRemoveEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.categorize() == EventCategory::BuffRemove
    }
}

/// Combat buff remove.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffRemove {
    /// Not used, different kind of event.
    None = 0,

    /// Last or all stacks removed.
    ///
    /// Sent by server.
    All = 1,

    /// Single stack removed.
    ///
    /// Happens for each stack on cleanse.
    ///
    /// Sent by server.
    Single = 2,

    /// Single stack removed.
    ///
    /// Automatically by Arc on out of combat or all stack.
    /// Ignore for strip/cleanse calculation.
    /// Use for in/out volume.
    Manual = 3,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}
