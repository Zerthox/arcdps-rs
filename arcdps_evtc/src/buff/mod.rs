mod condition;
mod formula;
mod info;

pub use self::condition::*;
pub use self::formula::*;
pub use self::info::*;

use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Combat buff remove.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffRemove {
    /// Not used, different kind of event.
    None = 0,

    /// Last or all stacks removed.
    ///
    /// Sent by server.
    All = 1,

    /// Single stack removed.
    ///
    /// Happens for each stack on cleanse.
    ///
    /// Sent by server.
    Single = 2,

    /// Single stack removed.
    ///
    /// Automatically by Arc on out of combat or all stack.
    /// Ignore for strip/cleanse calculation.
    /// Use for in/out volume.
    Manual = 3,

    /// Unknown or invalid.
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
