use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Attributes for buff formulas.
///
/// Used in [`StateChange::BuffFormula`](crate::StateChange::BuffFormula) events.
/// This enum is different from the game's own attribute ids.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u16)]
pub enum Attribute {
    None = 0,

    /// Power.
    Power = 1,

    /// Precision.
    Precision = 2,

    /// Toughness.
    Toughness = 3,

    /// Vitality.
    Vitality = 4,

    /// Ferocity.
    Ferocity = 5,

    /// Healing.
    Healing = 6,

    /// Condition Damage.
    Condition = 7,

    /// Concentration.
    Concentration = 8,

    /// Expertise.
    Expertise = 9,

    /// Armor.
    Armor = 10,

    /// Agony Resistance.
    Agony = 11,

    /// Stat increase.
    StatInc = 12,

    /// Outgoing strike damage.
    PhysInc = 13,

    /// Outgoing condition damage.
    CondInc = 14,

    /// Incoming strike damage.
    PhysRec = 15,

    /// Incoming condition damage.
    CondRec = 16,

    /// Attack speed.
    AttackSpeed = 17,

    /// Outgoing life leech.
    SiphonInc = 18,

    /// Incoming life leech.
    SiphonRec = 19,
}
