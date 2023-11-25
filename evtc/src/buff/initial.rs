use crate::{
    event::{impl_common, CommonEvent},
    extract::Extract,
    Event, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Buff initial event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffInitialEvent {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Current remaining duration.
    pub duration: i32,

    /// Original full duration.
    pub original_duration: i32,

    /// Whether stack is active.
    pub stack_active: bool,

    /// Buff stack (instance) id.
    pub stack_id: u32,
}

impl_common!(BuffInitialEvent);

impl Extract for BuffInitialEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            common: event.into(),
            duration: event.value,
            original_duration: event.buff_dmg,
            stack_active: event.is_shields != 0,
            stack_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for BuffInitialEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.is_buffinitial()
    }
}
