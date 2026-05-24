use crate::{
    Event, StateChange, TryExtract,
    event::{CommonEvent, impl_common},
    extract::Extract,
};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Buff remove single.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffRemoveSingle {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Buff remove kind.
    pub remove: BuffRemove,

    /// Buff duration.
    pub duration: i32,

    /// Buff stack (instance) id.
    pub stack_id: u32,
}

impl_common!(BuffRemoveSingle);

impl Extract for BuffRemoveSingle {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            common: event.into(),
            remove: event.get_buff_remove(),
            duration: event.value,
            stack_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for BuffRemoveSingle {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::BuffRemoveSingle
    }
}

/// Buff remvoe all.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffRemoveAll {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Buff remove kind.
    pub kind: BuffRemove,

    /// Removed duration as duration.
    pub duration: i32,

    /// Removed duration as intensity.
    pub duration_intensity: i32,
}

impl_common!(BuffRemoveAll);

impl Extract for BuffRemoveAll {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            common: event.into(),
            kind: event.get_buff_remove(),
            duration: event.value,
            duration_intensity: event.buff_dmg,
        }
    }
}

impl TryExtract for BuffRemoveAll {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::BuffRemoveAll
    }
}

/// Combat buff remove.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
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
