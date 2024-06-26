use crate::{
    event::{impl_common, CommonEvent},
    extract::Extract,
    Event, EventCategory, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Buff apply event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffApplyEvent {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Kind of buff application/extension.
    pub apply: BuffApplyKind,

    /// Stack active status.
    pub stack_active: u8,

    /// Buff stack (instance) id.
    pub stack_id: u32,
}

impl_common!(BuffApplyEvent);

impl Extract for BuffApplyEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            common: event.into(),
            apply: BuffApplyKind::extract(event),
            stack_active: event.is_shields,
            stack_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for BuffApplyEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.categorize() == EventCategory::BuffApply
    }
}

/// Buff apply behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum BuffApplyKind {
    /// New stack applied or existing stack replaced.
    Apply {
        /// Applied duration.
        duration: i32,

        /// Duration of removed stack, if any.
        removed_duration: u32,
    },

    /// Existing stack extended.
    Extend {
        /// New stack duration.
        new_duration: u32,

        /// Duration change.
        duration_change: i32,
    },
}

impl Extract for BuffApplyKind {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        if event.is_offcycle == 0 {
            Self::Apply {
                duration: event.value,
                removed_duration: event.overstack_value,
            }
        } else {
            Self::Extend {
                new_duration: event.overstack_value,
                duration_change: event.value,
            }
        }
    }
}
