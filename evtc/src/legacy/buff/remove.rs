use crate::{
    Event, TryExtract,
    buff::BuffRemove,
    event::{CommonEvent, impl_common},
    extract::Extract,
    legacy::LegacyEventCategory,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Buff remove event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffRemoveEvent {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Kind of buff remove.
    pub remove: BuffRemoveKind,

    /// Removed buff(s) as duration.
    pub removed_duration: i32,

    /// Removed buff(s) as intensity.
    ///
    /// **Warning:** may overflow on [`BuffRemove::All`].
    pub removed_intensity: i32,
}

impl_common!(BuffRemoveEvent);

impl Extract for BuffRemoveEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            common: event.into(),
            remove: unsafe { event.extract() },
            removed_duration: event.value,
            removed_intensity: event.buff_dmg,
        }
    }
}

impl TryExtract for BuffRemoveEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        LegacyEventCategory::from_event(event) == Some(LegacyEventCategory::BuffRemove)
    }
}

/// Kind of buff remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
pub enum BuffRemoveKind {
    /// Last or all stacks removed.
    ///
    /// Sent by server.
    All {
        /// Number of stacks removed.
        stacks_removed: u8,
    },

    /// Single stack removed.
    ///
    /// Happens for each stack on cleanse.
    ///
    /// Sent by server.
    Single {
        /// Stack (instance) id of removed buff.
        stack_id: u32,
    },

    /// Single stack removed.
    ///
    /// Automatically by Arc on out of combat or all stack.
    /// Ignore for strip/cleanse calculation.
    /// Use for in/out volume.
    Manual {
        /// Stack (instance) id of removed buff.
        stack_id: u32,
    },

    /// Unknown or invalid.
    Unknown(u8),
}

impl Extract for BuffRemoveKind {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        match event.get_buff_remove() {
            BuffRemove::None => unreachable!("extract buffremove on non-buffremove event"),
            BuffRemove::All => Self::All {
                stacks_removed: event.result,
            },
            BuffRemove::Single => Self::Single {
                stack_id: event.get_pad_id(),
            },
            BuffRemove::Manual => Self::Manual {
                stack_id: event.get_pad_id(),
            },
            BuffRemove::Unknown(value) => Self::Unknown(value),
        }
    }
}
