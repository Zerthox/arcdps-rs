mod activation;
mod info;
mod timing;

pub use self::activation::*;
pub use self::info::*;
pub use self::timing::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

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
