use crate::{CombatEvent, StateChange};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Buff formula from a [`CombatEvent`] with [`StateChange::BuffFormula`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffFormula {
    pub kind: u32,
    pub attr1: u32,
    pub attr2: u32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub trait_src: u32,
    pub trait_self: u32,
    pub buff_src: u32,
    pub buff_self: u32,
    pub not_npc: bool,
    pub not_player: bool,
    pub is_break: bool,
    pub value: u32,
    pub value_type: u8,
}

impl BuffFormula {
    /// Extracts buff information from a [`StateChange::BuffFormula`] event.
    ///
    /// # Safety
    /// This operation is safe when the [`CombatEvent`] is a valid buff formula event.
    #[inline]
    pub unsafe fn from_event(event: &CombatEvent) -> Self {
        RawBuffFormula::from_event(event).into()
    }

    /// Whether the buff formula is always active.
    #[inline]
    pub fn is_unconditional(&self) -> bool {
        self.trait_src == 0 && self.trait_self == 0 && self.buff_src == 0 && self.buff_self == 0
    }
}

impl From<RawBuffFormula> for BuffFormula {
    #[inline]
    fn from(raw: RawBuffFormula) -> Self {
        Self {
            kind: raw.kind as _,
            attr1: raw.attr1 as _,
            attr2: raw.attr2 as _,
            param1: raw.param1,
            param2: raw.param2,
            param3: raw.param3,
            trait_src: raw.trait_src as _,
            trait_self: raw.trait_self as _,
            buff_src: raw.buff_src as _,
            buff_self: raw.buff_self as _,
            not_npc: raw.not_npc,
            not_player: raw.not_player,
            is_break: raw.is_break,
            value: raw.value,
            value_type: raw.value_type,
        }
    }
}

impl TryFrom<&CombatEvent> for BuffFormula {
    type Error = ();

    #[inline]
    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        RawBuffFormula::try_from(event).map(Into::into)
    }
}

/// Buff formula from a [`CombatEvent`] with [`StateChange::BuffFormula`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawBuffFormula {
    pub kind: f32,
    pub attr1: f32,
    pub attr2: f32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub trait_src: f32,
    pub trait_self: f32,
    pub buff_src: f32,
    pub buff_self: f32,
    pub not_npc: bool,
    pub not_player: bool,
    pub is_break: bool,
    pub value: u32,
    pub value_type: u8,
}

impl RawBuffFormula {
    /// Extracts buff information from a [`StateChange::BuffFormula`] event.
    ///
    /// # Safety
    /// This operation is safe when the [`CombatEvent`] is a valid buff formula event.
    #[inline]
    pub unsafe fn from_event(event: &CombatEvent) -> Self {
        let [kind, attr1, attr2, param1, param2, param3, trait_src, trait_self]: [f32; 8] =
            transmute((
                event.time,
                event.src_agent,
                event.dst_agent,
                event.value,
                event.buff_dmg,
            ));
        let [buff_src, buff_self]: [f32; 2] = transmute((
            event.src_instance_id,
            event.dst_instance_id,
            event.src_master_instance_id,
            event.dst_master_instance_id,
        ));

        Self {
            kind,
            attr1,
            attr2,
            param1,
            param2,
            param3,
            trait_src,
            trait_self,
            buff_src,
            buff_self,
            not_npc: event.is_flanking != 0,
            not_player: event.is_shields != 0,
            is_break: event.is_off_cycle != 0,
            value: event.overstack_value,
            value_type: event.pad61,
        }
    }

    /// Whether the buff formula is always active.
    #[inline]
    pub fn is_unconditional(&self) -> bool {
        self.trait_src == 0.0
            && self.trait_self == 0.0
            && self.buff_src == 0.0
            && self.buff_self == 0.0
    }
}

impl TryFrom<&CombatEvent> for RawBuffFormula {
    type Error = ();

    #[inline]
    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::BuffFormula => Ok(unsafe { Self::from_event(event) }),
            _ => Err(()),
        }
    }
}

/// Attributes for buff formulas.
///
/// Used in [`StateChange::BuffFormula`](crate::StateChange::BuffFormula) events.
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
