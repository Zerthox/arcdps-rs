use crate::{
    Event, StateChange, TryExtract,
    event::{CommonEvent, impl_common},
    extract::Extract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Buff change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffChange {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Duration difference in milliseconds.
    pub duration_change: i32,

    /// New duration in milliseconds.
    pub new_duration: u32,

    /// Buff stack (instance) id.
    pub stack_id: u32,
}

impl_common!(BuffChange);

impl Extract for BuffChange {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            common: event.into(),
            duration_change: event.value,
            new_duration: event.overstack_value,
            stack_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for BuffChange {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::BuffChange
    }
}
