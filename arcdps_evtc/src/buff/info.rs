use crate::{CombatEvent, StateChange};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Buff information from a [`CombatEvent`] with [`StateChange::BuffInfo`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffInfo {
    /// The category of buff.
    ///
    /// See [`BuffCategory`] and [`BuffCategoryOld`].
    pub category: u8,

    /// Buff stacking behavior.
    ///
    /// See [`BuffStackType`].
    pub stacking_type: u8,

    /// Maximum amount of stacks.
    pub max_stacks: u16,

    /// Maximum buff duration.
    pub duration_cap: u32,

    /// Probably invulnerable.
    pub invulnerable: bool,

    /// Probably invert.
    pub invert: bool,

    /// Probably resistance.
    pub resistance: bool,
}

impl BuffInfo {
    /// Extracts buff information from a [`StateChange::BuffInfo`] event.
    #[inline]
    pub fn from_event(event: &CombatEvent) -> Self {
        Self {
            category: event.is_off_cycle,
            stacking_type: event.pad61,
            max_stacks: event.src_master_instance_id,
            duration_cap: event.overstack_value,
            invulnerable: event.is_flanking != 0,
            invert: event.is_shields != 0,
            resistance: event.pad62 != 0,
        }
    }
}

impl TryFrom<&CombatEvent> for BuffInfo {
    type Error = ();

    #[inline]
    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::BuffInfo => Ok(Self::from_event(event)),
            _ => Err(()),
        }
    }
}

/// Buff info category **after** 13 December 2022.
///
/// Used in [`StateChange::BuffInfo`](crate::StateChange::BuffInfo) events.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum BuffCategory {
    /// Buff is a Boon.
    Boon = 0,

    /// Buff is generic category.
    Any = 1,

    /// Buff is a Condition.
    Condition = 2,

    /// Buff is granted by Food consumable.
    Food = 5,

    /// Buff is a gear item or upgrade.
    Upgrade = 7,

    /// Buff is granted by a Boost consumable.
    Boost = 9,

    /// Buff is granted by a Trait.
    Trait = 12,

    /// Buff is a Transform.
    Transform = 13,

    /// Buff is Enhancement granted by a Utility consumable.
    Enhancement = 14,

    /// Buff is a Stance.
    Stance = 17,
}

/// Buff info category **before** 13 December 2022.
///
/// Used in [`StateChange::BuffInfo`](crate::StateChange::BuffInfo) events.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum BuffCategoryOld {
    /// Buff is a Boon.
    Boon = 0,

    /// Buff is generic category.
    Any = 1,

    /// Buff is a Condition.
    Condition = 2,

    /// Buff is granted by Food consumable.
    Food = 4,

    /// Buff is granted by gear item or upgrade.
    Upgrade = 6,

    /// Buff is granted by a Boost consumable.
    Boost = 8,

    /// Buff is granted by a Trait.
    Trait = 11,

    /// Buff is a Transform.
    Transform = 12,

    /// Buff is Enhancement granted by a Utility consumable.
    Enhancement = 13,

    /// Buff is a Stance.
    Stance = 16,
}

/// Buff stacking behavior.
///
/// Occurs in [`BuffInfo`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffStackType {
    /// Stacking in intensity with conditional loss.
    ///
    /// Similar to [`BuffStackType::Stacking`].
    StackingConditionalLoss = 0,

    /// Stacking in duration with queue.
    Queue = 1,

    /// Stacking in duration with cap.
    CappedDuration = 2,

    /// Regeneration-like stacking in duration.
    Regeneration = 3,

    /// Stacking in intensity.
    Stacking = 4,

    /// No stacking. Force override.
    Force = 5,
}
