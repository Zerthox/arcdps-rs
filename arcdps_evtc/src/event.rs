use crate::{Activation, Affinity, BuffRemove, StateChange};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// ArcDPS combat event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CombatEvent {
    /// `timeGetTime()` at time of registering the event.
    pub time: u64,

    /// Agent that caused the event.
    pub src_agent: usize,

    /// Agent the event happened to.
    pub dst_agent: usize,

    /// Value, if relevant to the event.
    pub value: i32,

    /// Buff damage, if relevant to the event.
    pub buff_dmg: i32,

    /// Overstack value, if relevant to the event.
    pub overstack_value: u32,

    /// Skill id of the relevant skill (can be zero).
    pub skill_id: u32,

    /// Id of source agent as appears in game at time of event.
    pub src_instance_id: u16,

    /// Id of destination agent as appears in game at time of event.
    pub dst_instance_id: u16,

    /// If `src_agent` has a master (e.g. is minion), will be equal to instance id of master, zero otherwise.
    pub src_master_instance_id: u16,

    /// If `dst_agent` has a master (e.g. is minion), will be equal to instance id of master, zero otherwise.
    pub dst_master_instance_id: u16,

    /// Current affinity of `src_agent` and `dst_agent`.
    ///
    /// *Arc calls this "iff" for if friend/foe.*
    pub affinity: Affinity,

    /// Buff, if relevant to the event.
    pub buff: u8,

    /// Combat result.
    ///
    /// For direct damage events this is a variant of [`Strike`](crate::Strike).
    pub result: u8,

    /// Whether event is a kind of [`Activation`].
    ///
    /// For [`Activation::CancelFire`] and [`Activation::CancelCancel`] `value` will be the ms duration of the time spent in animation.
    /// `buff_dmg` will be the ms duration of the scaled (as if not affected) time spent.
    ///
    /// For Normal or Quickness, `value` will be the ms duration at which all significant effects have happened.
    /// `buff_dmg` will be the ms duration at which control is expected to be returned to character.
    ///
    /// `dst_agent` will be x/y of target of skill effect.
    /// `overstack_value` will be z of target of skill effect.
    pub is_activation: Activation,

    /// Whether event is a kind of [`BuffRemove`].
    ///
    /// `src_agent` is agent that had buff removed, `dst_agent` is the agent that removed it.
    /// `value` will be the remaining time removed calculated as duration.
    /// `buff_dmg` will be the remaining time removed calculated as intensity.
    ///
    /// For [`BuffRemove::All`] `result` will be the number of stacks removed.
    /// For [`BuffRemove::Single`] pad61-64 (uint32) will be buff instance id of buff removed.
    pub is_buff_remove: BuffRemove,

    /// Whether `src_agent` is above 90% Health.
    pub is_ninety: u8,

    /// Whether `dst_agent` is below 50% Health.
    pub is_fifty: u8,

    /// Whether `src_agent` is moving at time of event.
    pub is_moving: u8,

    /// Whether event is a kind of [`StateChange`].
    pub is_statechange: StateChange,

    /// Whether `src_agent` is flanking at time of event.
    ///
    /// The value lies in a range of `1` to `135` degrees where `135` is rear.
    pub is_flanking: u8,

    /// Shields, if relevant to the event.
    pub is_shields: u8,

    /// For relevant Events this may be a variant of [`BuffCycle`](crate::BuffCycle).
    pub is_off_cycle: u8,

    /// Padding.
    ///
    /// May contain information depending on the kind of event.
    pub pad61: u8,

    /// Padding.
    ///
    /// May contain information depending on the kind of event.
    pub pad62: u8,

    /// Padding.
    ///
    /// May contain information depending on the kind of event.
    pub pad63: u8,

    /// Padding.
    ///
    /// May contain information depending on the kind of event.
    pub pad64: u8,
}

// TODO: conversion to rust-like enum?
impl CombatEvent {
    /// Determines the kind of event.
    pub fn kind(&self) -> EventKind {
        if self.is_statechange != StateChange::None {
            EventKind::StateChange
        } else if self.is_activation != Activation::None {
            EventKind::Activation
        } else if self.is_buff_remove != BuffRemove::None {
            EventKind::BuffRemove
        } else if self.buff != 0 {
            if self.buff_dmg == 0 {
                EventKind::BuffApply
            } else {
                EventKind::BuffDamage
            }
        } else {
            EventKind::DirectDamage
        }
    }
}

/// Possible [`CombatEvent`] kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
pub enum EventKind {
    StateChange,
    Activation,
    BuffRemove,
    BuffApply,
    BuffDamage,
    DirectDamage,
}

impl From<RawCombatEvent> for CombatEvent {
    fn from(raw: RawCombatEvent) -> Self {
        Self {
            time: raw.time,
            src_agent: raw.src_agent,
            dst_agent: raw.dst_agent,
            value: raw.value,
            buff_dmg: raw.buff_dmg,
            overstack_value: raw.overstack_value,
            skill_id: raw.skill_id,
            src_instance_id: raw.src_instance_id,
            dst_instance_id: raw.dst_instance_id,
            src_master_instance_id: raw.src_master_instance_id,
            dst_master_instance_id: raw.dst_master_instance_id,
            affinity: raw.affinity.into(),
            buff: raw.buff,
            result: raw.result,
            is_activation: raw.is_activation.into(),
            is_buff_remove: raw.is_buff_remove.into(),
            is_ninety: raw.is_ninety,
            is_fifty: raw.is_fifty,
            is_moving: raw.is_moving,
            is_statechange: raw.is_statechange.into(),
            is_flanking: raw.is_flanking,
            is_shields: raw.is_shields,
            is_off_cycle: raw.is_off_cycle,
            pad61: raw.pad61,
            pad62: raw.pad62,
            pad63: raw.pad63,
            pad64: raw.pad64,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RawCombatEvent {
    pub time: u64,
    pub src_agent: usize,
    pub dst_agent: usize,
    pub value: i32,
    pub buff_dmg: i32,
    pub overstack_value: u32,
    pub skill_id: u32,
    pub src_instance_id: u16,
    pub dst_instance_id: u16,
    pub src_master_instance_id: u16,
    pub dst_master_instance_id: u16,
    pub affinity: u8,
    pub buff: u8,
    pub result: u8,
    pub is_activation: u8,
    pub is_buff_remove: u8,
    pub is_ninety: u8,
    pub is_fifty: u8,
    pub is_moving: u8,
    pub is_statechange: u8,
    pub is_flanking: u8,
    pub is_shields: u8,
    pub is_off_cycle: u8,
    pub pad61: u8,
    pub pad62: u8,
    pub pad63: u8,
    pub pad64: u8,
}

impl From<CombatEvent> for RawCombatEvent {
    fn from(event: CombatEvent) -> Self {
        Self {
            time: event.time,
            src_agent: event.src_agent,
            dst_agent: event.dst_agent,
            value: event.value,
            buff_dmg: event.buff_dmg,
            overstack_value: event.overstack_value,
            skill_id: event.skill_id,
            src_instance_id: event.src_instance_id,
            dst_instance_id: event.dst_instance_id,
            src_master_instance_id: event.src_master_instance_id,
            dst_master_instance_id: event.dst_master_instance_id,
            affinity: event.affinity as u8,
            buff: event.buff,
            result: event.result,
            is_activation: event.is_activation as u8,
            is_buff_remove: event.is_buff_remove as u8,
            is_ninety: event.is_ninety,
            is_fifty: event.is_fifty,
            is_moving: event.is_moving,
            is_statechange: event.is_statechange as u8,
            is_flanking: event.is_flanking,
            is_shields: event.is_shields,
            is_off_cycle: event.is_off_cycle,
            pad61: event.pad61,
            pad62: event.pad62,
            pad63: event.pad63,
            pad64: event.pad64,
        }
    }
}
