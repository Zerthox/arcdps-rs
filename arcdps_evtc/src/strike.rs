use crate::{event::CommonEvent, CombatEvent};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// A direct damage (strike) event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StrikeEvent {
    #[serde(flatten)]
    pub common: CommonEvent,
    pub kind: Strike,
    pub total_damage: i32,
    pub shield_damage: u32,
    pub target_downed: bool,
}

impl TryFrom<&CombatEvent> for StrikeEvent {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            common: event.into(),
            kind: event.result.try_into().map_err(|_| ())?,
            total_damage: event.value,
            shield_damage: event.overstack_value,
            target_downed: event.is_offcycle == 1,
        })
    }
}

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
    Downed = 9,

    /// Skill dealt breakbar damage.
    ///
    /// Not a damage strike.
    Breakbar = 10,

    /// On-activation event.
    ///
    /// Not a damage strike.
    ///
    /// *Arc: Source hit target if damaging buff.*
    Activation = 11,
}

impl Strike {
    /// Whether the strike dealt health damage to the target.
    #[inline]
    pub const fn dealt_damage(&self) -> bool {
        matches!(self, Self::Normal | Self::Crit | Self::Glance)
    }
}
