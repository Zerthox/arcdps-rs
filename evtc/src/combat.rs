//! Bindings & utilities for any form of strikes (direct damage).

use crate::{
    Event, StateChange, TryExtract,
    event::{CommonEvent, impl_common},
    extract::Extract,
};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Combat event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CombatEvent {
    /// Common combat event information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonEvent,

    /// Combat event result.
    pub result: CombatResult,

    /// Is buff.
    pub is_buff: bool,

    /// Total strike damage inflicted.
    pub total_strike_damage: i32,

    /// Total buff damage inflicted.
    pub total_buff_damage: i32,

    /// Damage inflicted to shields (barrier).
    pub shield_damage: u32,

    /// Whether target is currently downed.
    pub target_downed: bool,
}

impl_common!(CombatEvent);

impl CombatEvent {
    #[inline]
    pub fn non_shield_strike_damage(&self) -> i32 {
        self.total_strike_damage - self.shield_damage as i32
    }

    #[inline]
    pub fn non_shield_buff_damage(&self) -> i32 {
        self.total_buff_damage - self.shield_damage as i32
    }
}

impl Extract for CombatEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            common: event.into(),
            result: event.get_combat_result(),
            is_buff: event.buff != 0,
            total_strike_damage: event.value,
            total_buff_damage: event.buff_dmg,
            shield_damage: event.overstack_value,
            target_downed: event.is_offcycle != 0,
        }
    }
}

impl TryExtract for CombatEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Combat
    }
}

/// Combat result.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum CombatResult {
    /// Strike damage.
    ///
    /// No critical hit, no glance.
    StrikeDamage = 0,

    /// Critical strike damage.
    StrikeDamageCrit = 1,

    /// Glancing strike damage.
    StrikeDamageGlance = 2,

    /// Attack was blocked.
    ///
    /// Due to Aegis, Chrono Shield 4 etc.
    Block = 3,

    /// Attack was evaded.
    ///
    /// Due to dodge, Mesmer Sword 2 etc.
    Evade = 4,

    /// Action was interrupted.
    Interrupt = 5,

    /// Attack was absorbed.
    ///
    /// Usually due to an invulnerability like Guardian Renewed Focus.
    Absorb = 6,

    /// Attack missed.
    ///
    /// Due to blind etc.
    Blind = 7,

    /// Attack killed the target.
    KillingBlow = 8,

    /// Attack downed the target.
    Downed = 9,

    /// Attack dealt breakbar damage.
    BreakbarDamage = 10,

    /// On-skill-activation event.
    SkillCast = 11,

    /// Skill crowd controlled the target.
    CrowdControl = 12,

    /// Damage was inverted.
    Invert = 13,

    /// Regular buff damage on cycle.
    BuffDamageCycle = 14,

    /// Buff damage outside of cycle.
    BuffDamageNotCycle = 15,

    /// Buff damage to target on hitting target (outside of cycle).
    BuffDamageTargetOnHit = 16,

    /// Buff damage to source on hitting target (outside of cycle).
    BuffDamageSourceOnHit = 17,

    /// Buff damage to target on buff removal (outside of cycle).
    BuffDamageOnBuffRemove = 18,

    /// Unknown.
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl CombatResult {
    /// Whether the attack dealt health damage to the target.
    #[inline]
    pub const fn is_health_damage(&self) -> bool {
        self.is_strike_damage() || self.is_buff_damage()
    }

    /// Whether the attack dealt strike damage to the target.
    #[inline]
    pub const fn is_strike_damage(&self) -> bool {
        matches!(
            self,
            Self::StrikeDamage | Self::StrikeDamageCrit | Self::StrikeDamageGlance
        )
    }

    /// Whether the attack dealt buff damage to the target.
    #[inline]
    pub const fn is_buff_damage(&self) -> bool {
        matches!(self, |Self::BuffDamageCycle| Self::BuffDamageNotCycle
            | Self::BuffDamageTargetOnHit
            | Self::BuffDamageSourceOnHit
            | Self::BuffDamageOnBuffRemove)
    }

    /// Whether the attack dealt breabkar damage to the target.
    #[inline]
    pub const fn is_breakbar_damage(&self) -> bool {
        matches!(self, Self::BreakbarDamage)
    }

    /// Whether the attack was prevented
    #[inline]
    pub const fn is_pervented(&self) -> bool {
        matches!(self, Self::Block | Self::Evade | Self::Absorb | Self::Blind)
    }
}
