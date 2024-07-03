use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Agent max health change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MaxHealthEvent {
    /// Time of registering the max health change.
    pub time: u64,

    /// Agent that had their max health changed.
    pub agent: AgentId,

    /// New agent max health.
    pub max_health: u64,
}

impl Extract for MaxHealthEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            max_health: event.dst_agent,
        }
    }
}

impl TryExtract for MaxHealthEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::MaxHealthUpdate
    }
}

/// Agent health percent change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HealthUpdateEvent {
    /// Time of registering the health percent change.
    pub time: u64,

    /// Agent that had their health percent changed.
    pub agent: AgentId,

    /// Current health percent with `1.0` being max.
    pub health: f32,
}

impl HealthUpdateEvent {
    /// Conversion ratio.
    pub const CONVERT: f32 = 10_000.0;
}

impl Extract for HealthUpdateEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            health: event.dst_agent as f32 / Self::CONVERT,
        }
    }
}

impl TryExtract for HealthUpdateEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::HealthUpdate
    }
}

/// Agent barrier percent change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BarrierUpdateEvent {
    /// Time of registering the barrier change.
    pub time: u64,

    /// Agent that had their current barrier changed.
    pub agent: AgentId,

    /// Current barrier percent with `1.0` being max.
    pub barrier: f32,
}

impl BarrierUpdateEvent {
    /// Conversion ratio.
    pub const CONVERT: f32 = 10_000.0;
}

impl Extract for BarrierUpdateEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            barrier: event.dst_agent as f32 / Self::CONVERT,
        }
    }
}

impl TryExtract for BarrierUpdateEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::BarrierUpdate
    }
}
