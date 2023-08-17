use std::mem::transmute;

use crate::{CombatEvent, CommonEvent, Extract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// A buff apply event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffApplyEvent {
    #[serde(flatten)]
    pub common: CommonEvent,
    pub buff: u8,
    pub duration: i32,
    pub kind: BuffApplyKind,
    pub stack_active: u8,
    pub instance_id: u32,
}

impl Extract for BuffApplyEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            common: event.into(),
            buff: event.buff,
            duration: event.value,
            kind: BuffApplyKind::extract(event),
            stack_active: event.is_shields,
            instance_id: transmute([event.pad61, event.pad62, event.pad63, event.pad64]),
        }
    }
}

/// Combat apply behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffApplyKind {
    Replace {
        removed_duration: u32,
    },
    Extend {
        previous_duration: u32,
        duration_change: i32,
    },
}

impl Extract for BuffApplyKind {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        if event.is_offcycle == 0 {
            Self::Replace {
                removed_duration: event.overstack_value,
            }
        } else {
            Self::Extend {
                previous_duration: event.overstack_value,
                duration_change: event.value,
            }
        }
    }
}
