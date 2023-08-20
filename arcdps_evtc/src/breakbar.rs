use crate::{extract::Extract, AgentId, CombatEvent, StateChange, TryExtract};
use num_enum::{FromPrimitive, IntoPrimitive};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Breakbar state changed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BreakbarStateEvent {
    pub time: u64,
    pub agent: AgentId,
    pub state: BreakbarState,
}

impl Extract for BreakbarStateEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            state: (event.value as u16).into(),
        }
    }
}

impl TryExtract for BreakbarStateEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::BreakbarState
    }
}

/// Breakbar percent changed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BreakbarPercentEvent {
    pub time: u64,
    pub agent: AgentId,
    pub health: f32,
}

impl Extract for BreakbarPercentEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        #[allow(clippy::transmute_int_to_float)]
        let health = transmute(event.value);

        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            health,
        }
    }
}

impl TryExtract for BreakbarPercentEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::BreakbarPercent
    }
}

/// Breakbar (defiance bar) states.
///
/// Occurs in [`StateChange::BreakbarState`](crate::StateChange::BreakbarState) events.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u16)]
pub enum BreakbarState {
    /// Defiance bar active.
    Active = 0,

    /// Defiance bar recovering.
    Recover = 1,

    /// Defiance bar immune.
    Immune = 2,

    /// No defiance.
    None = 3,

    /// Unknown state.
    #[num_enum(catch_all)]
    Unknown(u16),
}
