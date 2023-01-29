// workaround for strum derives on deprecated custom skill enum
#![allow(deprecated)]

mod buff;
mod event;
mod game;
mod state_change;

pub use buff::*;
pub use event::*;
pub use game::*;
pub use state_change::*;

use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Whether the agent is an ally or enemy.
///
/// *Arc calls this "iff" for if friend/foe.*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum Affinity {
    /// Allied agent.
    Friend,

    /// Enemy agent.
    Foe,

    /// Uncertain whether ally or enemy.
    #[num_enum(default)]
    Unknown,
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
    Normal,

    /// Strike was critical.
    Crit,

    /// Strike was glancing.
    Glance,

    /// Strike was blocked.
    ///
    /// Due to Aegis, Chrono Shield 4 etc.
    Block,

    /// Strike was evaded.
    ///
    /// Due to dodge, Mesmer Sword 2 etc.
    Evade,

    /// Strike interrupted something.
    Interrupt,

    /// Strike was absorbed.
    ///
    /// Usually due to an invulnerability like Guardian Renewed Focus.
    Absorb,

    /// Strike missed.
    ///
    /// Due to blind etc.
    Blind,

    /// Skill killed the target.
    ///
    /// Not a damage strike.
    KillingBlow,

    /// Skill downed the target.
    ///
    /// Not a damage strike.
    Downed,

    /// Skill dealt breakbar damage.
    ///
    /// Not a damage strike.
    Breakbar,

    /// On-activation event.
    ///
    /// Not a damage strike.
    ///
    /// *Arc: Source hit target if damaging buff.*
    Activation,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// Skill activation (cast).
///
/// *Arc calls this "combat activation".*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Activation {
    /// Not used, different kind of event.
    None,

    /// Started skill/animation activation.
    Start,

    /// Unused as of 5th November 2019.
    QuicknessUnused,

    /// Stopped skill activation with reaching tooltip time.
    CancelFire,

    /// Stopped skill activation without reaching tooltip time.
    CancelCancel,

    /// Animation completed fully.
    Reset,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// ArcDPS custom skill ids.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u16)]
pub enum CustomSkill {
    #[deprecated = "use `CustomSkill::RESURRECT` constant instead"]
    Resurrect = CustomSkill::RESURRECT as u16,

    #[deprecated = "use `CustomSkill::BANDAGE` constant instead"]
    Bandage = CustomSkill::BANDAGE as u16,

    #[deprecated = "use `CustomSkill::DODGE` constant instead"]
    Dodge = CustomSkill::DODGE as u16,
}

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

/// Content local for [`StateChange::IdToGUID`] events.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u32)]
pub enum ContentLocal {
    /// Content is an effect.
    Effect,

    /// Content is a marker.
    Marker,
}
