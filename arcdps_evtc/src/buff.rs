use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Combat buff remove.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffRemove {
    /// Not used, different kind of event.
    None,

    /// Last or all stacks removed.
    ///
    /// Sent by server.
    All,

    /// Single stack removed.
    ///
    /// Happens for each stack on cleanse.
    ///
    /// Sent by server.
    Single,

    /// Single stack removed.
    ///
    /// Automatically by Arc on out of combat or all stack.
    /// Ignore for strip/cleanse calculation.
    /// Use for in/out volume.
    Manual,

    /// Unknown or invalid.
    #[default]
    Unknown,
}

/// Combat buff cycle.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffCycle {
    /// Damage happened on tick timer.
    Cycle,

    /// Damage happened outside tick timer (resistable).
    NotCycle,

    /// Retired since May 2021.
    NotCycleOrResist,

    /// Damage happened to target on hitting target.
    NotCycleDmgToTargetOnHit,

    /// Damage happened to source on hitting target.
    NotCycleDmgToSourceOnHit,

    /// Damage happened to target on source losing a stack.
    NotCycleDmgToTargetOnStackRemove,

    #[default]
    Unknown,
}

/// Buff info category.
///
/// Used in [`StateChange::BuffInfo`](crate::StateChange::BuffInfo) events.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum BuffCategory {
    Boon = 0,
    Any = 1,
    Condition = 2,
    Food = 4,
    Upgrade = 6,
    Boost = 8,
    Trait = 11,
    Enhancement = 13,
    Stance = 16,
}

/// Attributes for buff formulas.
///
/// Used in [`StateChange::BuffFormula`](crate::StateChange::BuffFormula) events.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u16)]
pub enum Attribute {
    None,

    Power,
    Precision,
    Toughness,
    Vitality,
    Ferocity,
    Healing,
    Condition,
    Concentration,
    Expertise,
    Armor,

    /// Agony Resistance.
    Agony,

    /// Stat increase.
    StatInc,

    /// Flat Increase.
    FlatInc,

    /// Outgoing strike damage.
    PhysInc,

    /// Outgoing condition damage.
    CondInc,

    /// Incoming strike damage.
    PhysRec,

    /// Incoming condition damage.
    CondRec,

    /// Attack speed.
    Attackspeed,

    /// Outgoing life leech.
    SiphonInc,

    /// Incoming life leech.
    SiphonRec,

    /// Unknown or invalid.
    #[default]
    Unknown = u16::MAX,
}
