use crate::{CombatEvent, StateChange};
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Combat buff remove.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffRemove {
    /// Not used, different kind of event.
    None = 0,

    /// Last or all stacks removed.
    ///
    /// Sent by server.
    All = 1,

    /// Single stack removed.
    ///
    /// Happens for each stack on cleanse.
    ///
    /// Sent by server.
    Single = 2,

    /// Single stack removed.
    ///
    /// Automatically by Arc on out of combat or all stack.
    /// Ignore for strip/cleanse calculation.
    /// Use for in/out volume.
    Manual = 3,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Combat buff cycle.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffCycle {
    /// Damage happened on tick timer.
    Cycle = 0,

    /// Damage happened outside tick timer (resistable).
    NotCycle = 1,

    /// Retired since May 2021.
    NotCycleOrResist = 2,

    /// Damage happened to target on hitting target.
    NotCycleDmgToTargetOnHit = 3,

    /// Damage happened to source on hitting target.
    NotCycleDmgToSourceOnHit = 4,

    /// Damage happened to target on source losing a stack.
    NotCycleDmgToTargetOnStackRemove = 5,
}

/// Buff info category **before** 13 December 2022.
///
/// Used in [`StateChange::BuffInfo`](crate::StateChange::BuffInfo) events.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum BuffCategoryOld {
    /// Buff is a Boon.
    Boon = 0,

    /// Buff is generic category.
    Any = 1,

    /// Buff is a Condition.
    Condition = 2,

    /// Buff is granted by Food consumable.
    Food = 4,

    /// Buff is granted by gear item or upgrade.
    Upgrade = 6,

    /// Buff is granted by a Boost consumable.
    Boost = 8,

    /// Buff is granted by a Trait.
    Trait = 11,

    /// Buff is a Transform.
    Transform = 12,

    /// Buff is Enhancement granted by a Utility consumable.
    Enhancement = 13,

    /// Buff is a Stance.
    Stance = 16,
}

/// Buff info category **after** 13 December 2022.
///
/// Used in [`StateChange::BuffInfo`](crate::StateChange::BuffInfo) events.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum BuffCategory {
    /// Buff is a Boon.
    Boon = 0,

    /// Buff is generic category.
    Any = 1,

    /// Buff is a Condition.
    Condition = 2,

    /// Buff is granted by Food consumable.
    Food = 5,

    /// Buff is a gear item or upgrade.
    Upgrade = 7,

    /// Buff is granted by a Boost consumable.
    Boost = 9,

    /// Buff is granted by a Trait.
    Trait = 12,

    /// Buff is a Transform.
    Transform = 13,

    /// Buff is Enhancement granted by a Utility consumable.
    Enhancement = 14,

    /// Buff is a Stance.
    Stance = 17,
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

    /// Flat Increase.
    FlatInc = 13,

    /// Outgoing strike damage.
    PhysInc = 14,

    /// Outgoing condition damage.
    CondInc = 15,

    /// Incoming strike damage.
    PhysRec = 16,

    /// Incoming condition damage.
    CondRec = 17,

    /// Attack speed.
    Attackspeed = 18,

    /// Outgoing life leech.
    SiphonInc = 19,

    /// Incoming life leech.
    SiphonRec = 20,
}

/// Buff information from a [`CombatEvent`] with [`StateChange::BuffInfo`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffInfo {
    pub category: u8,
    pub stacking_type: u8,
    pub max_stacks: u16,
    pub duration_cap: u32,

    /// Probably invulnerable.
    pub invulnerable: bool,

    /// Probably invulnerable.
    pub invert: bool,

    /// Probably resistance.
    pub resistance: bool,
}

impl TryFrom<&CombatEvent> for BuffInfo {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::BuffInfo => Ok(Self {
                category: event.is_off_cycle,
                stacking_type: event.pad61,
                max_stacks: event.src_master_instance_id,
                duration_cap: event.overstack_value,
                invulnerable: event.is_flanking != 0,
                invert: event.is_shields != 0,
                resistance: event.pad62 != 0,
            }),

            _ => Err(()),
        }
    }
}

/// Buff formula from a [`CombatEvent`] with [`StateChange::BuffFormula`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffFormula {
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

impl TryFrom<&CombatEvent> for BuffFormula {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::BuffFormula => {
                let [kind, attr1, attr2, param1, param2, param3, trait_src, trait_self]: [f32; 8] = unsafe {
                    transmute((
                        event.time,
                        event.src_agent,
                        event.dst_agent,
                        event.value,
                        event.buff_dmg,
                    ))
                };
                let [buff_src, buff_self]: [f32; 2] = unsafe {
                    transmute((
                        event.src_instance_id,
                        event.dst_instance_id,
                        event.src_master_instance_id,
                        event.dst_master_instance_id,
                    ))
                };

                Ok(Self {
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
                })
            }

            _ => Err(()),
        }
    }
}
