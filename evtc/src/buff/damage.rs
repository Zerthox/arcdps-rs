use crate::{
    event::{impl_common, CommonEvent},
    extract::Extract,
    Event, EventCategory, TryExtract,
};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Buff damage event.
///
/// For example from a Condition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffDamageEvent {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Buff.
    // TODO: meaning?
    pub buff: u8,

    /// Buff damage amount.
    pub damage: i32,

    /// Whether damage happened on tick (cycle) or reactively (off-cycle).
    pub cycle: BuffCycle,

    /// Result of buff damage.
    pub result: BuffDamageResult,
}

impl_common!(BuffDamageEvent);

impl Extract for BuffDamageEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            common: event.into(),
            buff: event.buff,
            damage: event.buff_dmg,
            cycle: event.is_offcycle.into(),
            result: event.result.into(),
        }
    }
}

impl TryExtract for BuffDamageEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.categorize() == EventCategory::BuffDamage
    }
}

/// Buff damage tick results.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum BuffDamageResult {
    /// Expected to hit.
    Hit = 0,

    /// Target invulnerable by buff.
    InvulnByBuff = 1,

    /// Target invulnerable by player skill.
    InvulnBySkill1 = 2,

    /// Target invulnerable by player skill.
    InvulnBySkill2 = 3,

    /// Target invulnerable by player skill.
    InvulnBySkill3 = 4,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Combat buff cycle.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum BuffCycle {
    /// Damage happened on tick timer.
    Cycle = 0,

    /// Damage happened outside tick timer (resistable).
    NotCycle = 1,

    /// Retired since May 2021.
    NotCycleOrResist = 2,

    /// Damage happened to target on hitting target.
    NotCycleDmgToTargetOnHit = 3,

    /// Damage happened to source on hitting target.
    NotCycleDmgToSourceOnHit = 4,

    /// Damage happened to target on source losing a stack.
    NotCycleDmgToTargetOnStackRemove = 5,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}
