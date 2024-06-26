use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Breakbar state change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BreakbarStateEvent {
    /// Time of registering the breakbar state change.
    pub time: u64,

    /// Agent changing breakbar state.
    pub agent: AgentId,

    /// New breakbar state.
    pub state: BreakbarState,
}

impl Extract for BreakbarStateEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            state: (event.value as u16).into(),
        }
    }
}

impl TryExtract for BreakbarStateEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::BreakbarState
    }
}

/// Breakbar percent change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BreakbarPercentEvent {
    /// Time of registering the breakbar health change.
    pub time: u64,

    /// Agent that had their breakbar health changed.
    pub agent: AgentId,

    /// New breakbar health in percent.
    pub health: f32,
}

impl Extract for BreakbarPercentEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let health = f32::from_ne_bytes(event.value.to_ne_bytes());

        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            health,
        }
    }
}

impl TryExtract for BreakbarPercentEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::BreakbarPercent
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
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
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
