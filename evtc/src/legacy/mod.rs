//! Legacy events and types.

mod activation;
mod buff;

pub use self::{activation::*, buff::*};

use crate::{Event, EventKind, StateChange};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Legacy event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LegacyEventKind {
    /// Activation (cast) event.
    Activation(ActivationEvent),

    /// Buff removed.
    BuffRemove(BuffRemoveEvent),

    /// Buff applied.
    BuffApply(BuffApplyEvent),

    /// Buff damage.
    BuffDamage(BuffDamageEvent),

    /// Regular event.
    Event(EventKind),
}

impl LegacyEventKind {
    #[inline]
    pub fn is_legacy(event: &Event) -> bool {
        LegacyEventCategory::from_event(event).is_some()
    }
}

impl From<Event> for LegacyEventKind {
    #[inline]
    fn from(event: Event) -> Self {
        unsafe {
            match LegacyEventCategory::from_event(&event) {
                Some(LegacyEventCategory::Activation) => Self::Activation(event.extract()),
                Some(LegacyEventCategory::BuffRemove) => Self::BuffRemove(event.extract()),
                Some(LegacyEventCategory::BuffApply) => Self::BuffApply(event.extract()),
                Some(LegacyEventCategory::BuffDamage) => Self::BuffDamage(event.extract()),
                None => Self::Event(event.into()),
            }
        }
    }
}

/// Legacy event categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LegacyEventCategory {
    Activation,
    BuffRemove,
    BuffApply,
    BuffDamage,
}

impl LegacyEventCategory {
    #[inline]
    pub fn from_event(event: &Event) -> Option<Self> {
        let statechange = event.get_statechange();
        if statechange == StateChange::Combat
            || (statechange == StateChange::BuffInitial && event.buff != 18)
        {
            if Activation::from_event(event) != Activation::None {
                Some(Self::Activation)
            } else if BuffRemove::from_event(event) != BuffRemove::None {
                Some(Self::BuffRemove)
            } else if event.buff != 0 {
                Some(if event.buff_dmg == 0 && event.value != 0 {
                    Self::BuffApply
                } else {
                    Self::BuffDamage
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
