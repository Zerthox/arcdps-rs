//! Event bindings & utilities.

mod category;
mod common;
mod event_kind;
mod old;

pub use self::category::*;
pub use self::common::*;
pub use self::event_kind::*;
pub use self::old::*;

pub use crate::{
    agent::{
        AgentStatusEvent, AttackTargetEvent, BarrierUpdateEvent, BreakbarPercentEvent,
        BreakbarStateEvent, DownContributionEvent, EnterCombatEvent, HealthUpdateEvent,
        MaxHealthEvent, TargetableEvent, TeamChangeEvent,
    },
    buff::{
        BuffApplyEvent, BuffDamageEvent, BuffFormula, BuffInfo, BuffRemoveEvent, StackActiveEvent,
        StackResetEvent,
    },
    effect::{Effect, EffectGUID, EffectOld},
    log::{ErrorEvent, LogEvent},
    player::{GuildEvent, RewardEvent, TagEvent},
    position::PositionEvent,
    skill::{ActivationEvent, SkillInfo, SkillTiming},
    strike::StrikeEvent,
    weapon::WeaponSwapEvent,
};

use crate::{
    buff::{BuffCycle, BuffRemove},
    extract::Extract,
    skill::Activation,
    strike::Strike,
    Affinity, StateChange, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ArcDPS event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Event {
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
    /// Use [`Event::get_affinity`] to obtain the value as [`Affinity`].
    ///
    /// *Arc calls this "iff" for if friend/foe.*
    pub affinity: u8,

    /// Buff, if relevant to the event.
    pub buff: u8,

    /// Combat result.
    ///
    /// For strike (direct damage) events this contains the kind of strike.
    ///
    /// Use [`Event::get_strike`] to obtain the value as [`Strike`].
    pub result: u8,

    /// Whether event is a kind of activation event.
    ///
    /// Use [`Event::get_activation`] to obtain the value as [`Activation`].
    ///
    /// For [`Activation::CancelFire`] and [`Activation::CancelCancel`] `value` will be the ms duration of the time spent in animation.
    /// `buff_dmg` will be the ms duration of the scaled (as if not affected) time spent.
    ///
    /// For Normal or Quickness, `value` will be the ms duration at which all significant effects have happened.
    /// `buff_dmg` will be the ms duration at which control is expected to be returned to character.
    ///
    /// `dst_agent` will be x/y of target of skill effect.
    /// `overstack_value` will be z of target of skill effect.
    pub is_activation: u8,

    /// Whether event is a kind of buff remove event.
    ///
    /// Use [`Event::get_buffremove`] to obtain the value as [`BuffRemove`].
    ///
    /// `src_agent` is agent that had buff removed, `dst_agent` is the agent that removed it.
    /// `value` will be the remaining time removed calculated as duration.
    /// `buff_dmg` will be the remaining time removed calculated as intensity.
    ///
    /// For [`BuffRemove::All`] `result` will be the number of stacks removed.
    /// For [`BuffRemove::Single`] pad61-64 (uint32) will be buff instance id of buff removed.
    pub is_buffremove: u8,

    /// Whether `src_agent` is above 90% Health.
    pub is_ninety: u8,

    /// Whether `dst_agent` is below 50% Health.
    pub is_fifty: u8,

    /// Whether `src_agent` is moving at time of event.
    pub is_moving: u8,

    /// Whether event is a kind of state change event.
    ///
    /// Use [Event::get_statechange] to obtain the value as [`StateChange`].
    pub is_statechange: u8,

    /// Whether `src_agent` is flanking at time of event.
    ///
    /// The value lies in a range of `1` to `135` degrees where `135` is rear.
    pub is_flanking: u8,

    /// Shields, if relevant to the event.
    pub is_shields: u8,

    /// For relevant events this contains when the buff cycle happened.
    ///
    /// Use [`Event::get_buffcycle`] to obtain the value as [`BuffCycle`].
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

impl Event {
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

    /// Returns the event `is_statechange` as [`StateChange`].
    #[inline]
    pub fn get_statechange(&self) -> StateChange {
        self.is_statechange.into()
    }

    /// Returns the event `affinity` as [`Affinity`].
    ///
    /// This will return [`Affinity::Unknown`] if the event has no valid data in `affinity`.
    #[inline]
    pub fn get_affinity(&self) -> Affinity {
        self.affinity.into()
    }

    /// Returns the event `is_activation` as [`Activation`].
    ///
    /// This will return [`Activation::Unknown`] if the event has no valid data in `is_activation`.
    #[inline]
    pub fn get_activation(&self) -> Activation {
        self.is_activation.into()
    }

    /// Returns the event `is_buffremove` as [`BuffRemove`].
    ///
    /// This will return [`BuffRemove::Unknown`] if the event has no valid data in `is_buffremove`.
    #[inline]
    pub fn get_buffremove(&self) -> BuffRemove {
        self.is_buffremove.into()
    }

    /// Returns the event `result` as [`Strike`].
    ///
    /// This will return [`Strike::Unknown`] if the event has no valid data in `result`.
    #[inline]
    pub fn get_strike(&self) -> Strike {
        self.result.into()
    }

    /// Returns the event `is_offcycle` as [`BuffCycle`].
    ///
    /// This will return [`BuffCycle::Unknown`] if the event has no valid data in `is_offcycle`.
    #[inline]
    pub fn get_buffcycle(&self) -> BuffCycle {
        self.is_offcycle.into()
    }

    /// Checks whether the event has a timestamp.
    #[inline]
    pub fn has_time(&self) -> bool {
        self.get_statechange().has_time()
    }

    /// Retrieves the event time, if present.
    #[inline]
    pub fn time(&self) -> Option<u64> {
        self.has_time().then_some(self.time)
    }

    /// Forcefully extracts a type implementing [`Extract`] from the event.
    ///
    /// # Safety
    /// This is safe when the given event is a valid event to extract the type from.
    #[inline]
    pub unsafe fn extract<T>(&self) -> T
    where
        T: Extract,
    {
        T::extract(self)
    }

    /// Attempts to extract a type implementing [`TryExtract`] from the event.
    #[inline]
    pub fn try_extract<T>(&self) -> Option<T>
    where
        T: TryExtract,
    {
        T::try_extract(self)
    }

    /// Attempts to extract an [`ActivationEvent`] from the event.
    #[inline]
    pub fn to_activation(&self) -> Option<ActivationEvent> {
        self.try_extract()
    }

    /// Attempts to extract a [`BuffRemoveEvent`] from the event.
    #[inline]
    pub fn to_buff_remove(&self) -> Option<BuffRemoveEvent> {
        self.try_extract()
    }

    /// Attempts to extract a [`BuffApplyEvent`] from the event.
    #[inline]
    pub fn to_buff_apply(&self) -> Option<BuffApplyEvent> {
        self.try_extract()
    }

    /// Attempts to extract a [`BuffDamageEvent`] from the event.
    #[inline]
    pub fn to_buff_damage(&self) -> Option<BuffDamageEvent> {
        self.try_extract()
    }

    /// Attempts to extract a [`StrikeEvent`] from the event.
    #[inline]
    pub fn to_strike(&self) -> Option<StrikeEvent> {
        self.try_extract()
    }
}
