use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Strike types.
///
/// *Arc calls this "combat result".*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
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
    Normal = 0,

    /// Strike was critical.
    Crit = 1,

    /// Strike was glancing.
    Glance = 2,

    /// Strike was blocked.
    ///
    /// Due to Aegis, Chrono Shield 4 etc.
    Block = 3,

    /// Strike was evaded.
    ///
    /// Due to dodge, Mesmer Sword 2 etc.
    Evade = 4,

    /// Strike interrupted something.
    Interrupt = 5,

    /// Strike was absorbed.
    ///
    /// Usually due to an invulnerability like Guardian Renewed Focus.
    Absorb = 6,

    /// Strike missed.
    ///
    /// Due to blind etc.
    Blind = 7,

    /// Skill killed the target.
    ///
    /// Not a damage strike.
    KillingBlow = 8,

    /// Skill downed the target.
    ///
    /// Not a damage strike.
    Downed = 10,

    /// Skill dealt breakbar damage.
    ///
    /// Not a damage strike.
    Breakbar = 11,

    /// On-activation event.
    ///
    /// Not a damage strike.
    ///
    /// *Arc: Source hit target if damaging buff.*
    Activation = 12,
}
