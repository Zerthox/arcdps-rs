use crate::{
    Event, StateChange, TryExtract,
    event::{CommonEvent, impl_common},
    extract::Extract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Buff remove single.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffRemoveSingle {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

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
