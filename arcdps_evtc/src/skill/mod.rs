mod info;
mod timing;

pub use self::info::*;
pub use self::timing::*;

use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Skill activation (cast).
///
/// *Arc calls this "combat activation".*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum Activation {
    /// Not used, different kind of event.
    None = 0,

    /// Started skill/animation activation.
    Start = 1,

    /// Unused as of 5th November 2019.
    QuicknessUnused = 2,

    /// Stopped skill activation with reaching tooltip time.
    CancelFire = 3,

    /// Stopped skill activation without reaching tooltip time.
    CancelCancel = 4,

    /// Animation completed fully.
    Reset = 5,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// ArcDPS custom skill ids.
pub enum CustomSkill {}

impl CustomSkill {
    /// Resurrect skill.
    ///
    /// Not custom but important and unnamed.
    pub const RESURRECT: u32 = 1066;

    /// Bandage downstate skill.
    ///v
    /// Personal healing only.
    pub const BANDAGE: u32 = 1175;

    /// Dodge skill.
    ///
    /// Will occur in `is_activation == normal` event.
    pub const DODGE: u32 = 65001;
}

/// Agent weapon set.
///
/// Typically used with a [`CombatEvent`] with [`StateChange::WeaponSwap`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u64)]
pub enum WeaponSet {
    /// First underwater weapon set.
    Water1 = 0,

    /// Second underwater weapon set.
    Water2 = 1,

    /// Bundle or kit weapon set.
    Bundle = 2,

    /// Transform weapon set.
    Transform = 3,

    /// First land weapon set.
    Land1 = 4,

    /// Second land weapon set.
    Land2 = 5,
}
