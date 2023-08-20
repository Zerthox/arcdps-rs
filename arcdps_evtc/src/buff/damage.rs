use crate::{event::CommonEvent, extract::Extract, CombatEvent, EventCategory, TryExtract};
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// A buff damage event.
///
/// For example from a Condition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffDamageEvent {
    #[serde(flatten)]
    pub common: CommonEvent,
    pub buff: u8,
    pub damage: i32,
    pub on_tick: bool,
    pub result: BuffDamageResult,
}

impl Extract for BuffDamageEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            common: event.into(),
            buff: event.buff,
            damage: event.buff_dmg,
            on_tick: event.is_offcycle == 0,
            result: event.result.into(),
        }
    }
}

impl TryExtract for BuffDamageEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
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
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
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

    /// Unknown.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Combat buff cycle.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
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
}
