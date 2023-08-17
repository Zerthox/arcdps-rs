use crate::{AgentId, CombatEvent, Extract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Simple event regarding a specific agent.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentStatusEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that the event happened to.
    pub agent: AgentId,
}

impl Extract for AgentStatusEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
        }
    }
}

/// Agent entered combat.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnterCombatEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that the state change happened to.
    pub agent: AgentId,

    /// Agent subgroup.
    pub subgroup: u64,
}

impl Extract for EnterCombatEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            subgroup: event.dst_agent,
        }
    }
}

/// Agent health.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MaxHealthEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that the state change happened to.
    pub agent: AgentId,

    /// Agent health.
    pub health: u64,
}

impl Extract for MaxHealthEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            health: event.dst_agent,
        }
    }
}

/// Agent health percent change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HealthUpdateEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that the state change happened to.
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
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            health: event.dst_agent as f32 / Self::CONVERT,
        }
    }
}

/// Agent barrier percent change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BarrierUpdateEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that the state change happened to.
    pub agent: AgentId,

    /// Current barrier percent with `1.0` being max.
    pub health: f32,
}

impl BarrierUpdateEvent {
    /// Conversion ratio.
    pub const CONVERT: f32 = 10_000.0;
}

impl Extract for BarrierUpdateEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            health: event.dst_agent as f32 / Self::CONVERT,
        }
    }
}
