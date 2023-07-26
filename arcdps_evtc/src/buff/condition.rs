use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Condition tick results.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum ConditionResult {
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
}
