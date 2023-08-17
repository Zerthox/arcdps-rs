mod category;
mod common;
mod kind;
mod old;
mod raw;

pub use self::category::*;
pub use self::common::*;
pub use self::kind::*;
pub use self::old::*;
pub use self::raw::*;

use crate::{Activation, Affinity, BuffRemove, Position, StateChange};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ArcDPS combat event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CombatEvent {
    /// `timeGetTime()` at time of registering the event.
    pub time: u64,

    /// Agent that caused the event.
    pub src_agent: u64,

    /// Agent the event happened to.
    pub dst_agent: u64,

    /// Value, if relevant to the event.
    pub value: i32,

    /// Buff damage, if relevant to the event.
    pub buff_dmg: i32,

    /// Overstack value, if relevant to the event.
    pub overstack_value: u32,

    /// Skill id of the relevant skill (can be zero).
    pub skill_id: u32,

    /// Instance id of source agent as appears in game at time of event.
    pub src_instance_id: u16,

    /// Instance id of destination agent as appears in game at time of event.
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
    pub is_buffremove: BuffRemove,

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

    /// For relevant events this may be a variant of [`BuffCycle`](crate::BuffCycle).
    pub is_offcycle: u8,

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

impl CombatEvent {
    /// Determines the [`EventCategory`] of the event.
    #[inline]
    pub fn categorize(&self) -> EventCategory {
        self.into()
    }

    /// Converts the event into its [`EventKind`] representation.
    #[inline]
    pub fn into_kind(self) -> EventKind {
        self.into()
    }

    /// Checks whether the event has a timestamp.
    #[inline]
    pub fn has_time(&self) -> bool {
        self.is_statechange.has_time()
    }

    /// Retrieves the event time, if present.
    #[inline]
    pub fn time(&self) -> Option<u64> {
        self.has_time().then_some(self.time)
    }

    /// Attempts to extract [`Position`] data from the event.
    #[inline]
    pub fn position(&self) -> Option<Position> {
        self.try_into().ok()
    }
}

impl From<RawCombatEvent> for CombatEvent {
    #[inline]
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
            is_buffremove: raw.is_buffremove.into(),
            is_ninety: raw.is_ninety,
            is_fifty: raw.is_fifty,
            is_moving: raw.is_moving,
            is_statechange: raw.is_statechange.into(),
            is_flanking: raw.is_flanking,
            is_shields: raw.is_shields,
            is_offcycle: raw.is_offcycle,
            pad61: raw.pad61,
            pad62: raw.pad62,
            pad63: raw.pad63,
            pad64: raw.pad64,
        }
    }
}
