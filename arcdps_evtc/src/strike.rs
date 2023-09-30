//! Bindings & utilities for any form of strikes (hits).

use crate::{event::CommonEvent, extract::Extract, Event, EventCategory, TryExtract};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Direct damage (strike) event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StrikeEvent {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Kind of strike.
    pub kind: Strike,

    /// Total damage inflicted.
    pub total_damage: i32,

    /// Damage inflicted to shields (barrier).
    pub shield_damage: u32,

    /// Whether target is currently downed.
    pub target_downed: bool,
}

impl Extract for StrikeEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            common: event.into(),
            kind: event.result.into(),
            total_damage: event.value,
            shield_damage: event.overstack_value,
            target_downed: event.is_offcycle == 1,
        }
    }
}

impl TryExtract for StrikeEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.categorize() == EventCategory::Strike
    }
}

/// Strike types.
///
/// *Arc calls this "combat result".*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum Strike {
    /// Normal damage strike.
    ///
    /// No crit, no glance.
    Normal = 0,

    /// Strike was critical.
    Crit = 1,

    /// Strike was glancing.
    Glance = 2,

    /// Strike was blocked.
    ///
    /// Due to Aegis, Chrono Shield 4 etc.
    Block = 3,

    /// Strike was evaded.
    ///
    /// Due to dodge, Mesmer Sword 2 etc.
    Evade = 4,

    /// Strike interrupted something.
    Interrupt = 5,

    /// Strike was absorbed.
    ///
    /// Usually due to an invulnerability like Guardian Renewed Focus.
    Absorb = 6,

    /// Strike missed.
    ///
    /// Due to blind etc.
    Blind = 7,

    /// Skill killed the target.
    ///
    /// Not a damage strike.
    KillingBlow = 8,

    /// Skill downed the target.
    ///
    /// Not a damage strike.
    Downed = 9,

    /// Skill dealt breakbar damage.
    ///
    /// Not a damage strike.
    Breakbar = 10,

    /// On-activation event.
    ///
    /// Not a damage strike.
    ///
    /// *Arc: Source hit target if damaging buff.*
    Activation = 11,

    /// Unknown.
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl Strike {
    /// Whether the strike dealt health damage to the target.
    #[inline]
    pub const fn dealt_damage(&self) -> bool {
        matches!(self, Self::Normal | Self::Crit | Self::Glance)
    }
}
