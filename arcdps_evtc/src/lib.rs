//! Bindings for the ArcDPS EVTC API.
//!
//! Includes everything shared between Arc's realtime API used by plugins and Arc's log API consumed by parsers.

mod agent;
mod buff;
mod effect;
mod event;
mod game;
mod position;
mod skill;
mod state_change;

pub use self::agent::*;
pub use self::buff::*;
pub use self::effect::*;
pub use self::event::*;
pub use self::game::*;
pub use self::position::*;
pub use self::skill::*;
pub use self::state_change::*;

use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Whether the agent is an ally or enemy.
///
/// *Arc calls this "iff" for if friend/foe.*
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
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
    #[default]
    Unknown,
}

/// Strike types.
///
/// *Arc calls this "combat result".*
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
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
    #[default]
    Unknown,
}

/// Skill activation (cast).
///
/// *Arc calls this "combat activation".*
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
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
    #[default]
    Unknown,
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
