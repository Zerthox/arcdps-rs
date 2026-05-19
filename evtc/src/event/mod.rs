//! Event bindings & utilities.

mod common;
mod event_kind;
mod state_change;

pub use self::{common::*, event_kind::*, state_change::*};

pub use crate::{
    agent::{
        AgentStatusEvent, AttackTargetEvent, BarrierUpdateEvent, BreakbarPercentEvent,
        BreakbarStateEvent, DownContributionEvent, EnterCombatEvent, GliderEvent,
        HealthUpdateEvent, MaxHealthEvent, StunbreakEvent, TargetableEvent, TeamChangeEvent,
    },
    buff::{
        BuffApply, BuffChange, BuffFormula, BuffInfo, BuffInitialEvent, BuffRemoveAll,
        BuffRemoveSingle, StackActiveEvent, StackResetEvent,
    },
    combat::CombatEvent,
    effect::{
        AgentEffect, AgentEffectRemove, Effect45, Effect51, GroundEffect, GroundEffectRemove,
    },
    log::{ArcBuildEvent, ErrorEvent, LogEvent},
    marker::{AgentMarkerEvent, SquadMarkerEvent},
    missile::{MissileCreate, MissileLaunch, MissileRemove},
    player::{GuildEvent, RewardEvent},
    position::PositionEvent,
    skill::{SkillInfo, SkillTiming},
    weapon::WeaponSwapEvent,
};

use crate::{Affinity, CombatResult, TryExtract, extract::Extract, legacy::LegacyEventKind};

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

    /// Combat result for combat events.
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
    /// Converts the event into its [`EventKind`] representation.
    #[inline]
    pub fn into_kind(self) -> EventKind {
        self.into()
    }

    /// Converts the event into its [`LegacyEventKind`] representation.
    #[inline]
    pub fn into_legacy(self) -> LegacyEventKind {
        self.into()
    }

    /// Checks whether the event is a legacy event.
    #[inline]
    pub fn is_legacy(&self) -> bool {
        LegacyEventKind::is_legacy(self)
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

    /// Returns the padding as [`u32`] id/signature.
    #[inline]
    pub fn get_pad_id(&self) -> u32 {
        u32::from_le_bytes([self.pad61, self.pad62, self.pad63, self.pad64])
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
        unsafe { T::extract(self) }
    }

    /// Attempts to extract a type implementing [`TryExtract`] from the event.
    #[inline]
    pub fn try_extract<T>(&self) -> Option<T>
    where
        T: TryExtract,
    {
        T::try_extract(self)
    }

    /// Checks whether the event is an initial buff event.
    #[inline]
    pub fn is_buffinitial(&self) -> bool {
        self.get_statechange() == StateChange::BuffInitial && self.buff == 18
    }

    /// Returns the result as [`CombatResult`].
    #[inline]
    pub fn get_combat_result(&self) -> CombatResult {
        self.result.into()
    }

    /// Checks whether the source is moving, if applicable for this event type.
    #[inline]
    pub fn is_source_moving(&self) -> bool {
        (self.is_moving & 1) != 0
    }

    /// Checks whether the target is moving, if applicable for this event type.
    #[inline]
    pub fn is_target_moving(&self) -> bool {
        (self.is_moving & 2) != 0
    }
}
