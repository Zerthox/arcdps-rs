use crate::{extract::Extract, AgentId, CombatEvent, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Simple event regarding an agent.
///
/// The meaning depends on the context.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentStatusEvent {
    /// Time of registering the status change.
    pub time: u64,

    /// Agent that the status change happened to.
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

impl TryExtract for AgentStatusEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        matches!(
            event.is_statechange,
            StateChange::ExitCombat
                | StateChange::ChangeUp
                | StateChange::ChangeDead
                | StateChange::ChangeDown
                | StateChange::Spawn
                | StateChange::Despawn
                | StateChange::PointOfView
        )
    }
}

/// Agent entered combat.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnterCombatEvent {
    /// Time of registering the combat enter.
    pub time: u64,

    /// Agent that entered combat.
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

impl TryExtract for EnterCombatEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::EnterCombat
    }
}

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
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            max_health: event.dst_agent,
        }
    }
}

impl TryExtract for MaxHealthEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::MaxHealthUpdate
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
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            health: event.dst_agent as f32 / Self::CONVERT,
        }
    }
}

impl TryExtract for HealthUpdateEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::HealthUpdate
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
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            barrier: event.dst_agent as f32 / Self::CONVERT,
        }
    }
}

impl TryExtract for BarrierUpdateEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::BarrierUpdate
    }
}

/// Agent team change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TeamChangeEvent {
    /// Time of registering the team change.
    pub time: u64,

    /// Agent that had their team changed.
    pub agent: AgentId,

    /// New team id.
    pub team: u64,
}

impl Extract for TeamChangeEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            team: event.dst_agent,
        }
    }
}

impl TryExtract for TeamChangeEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::TeamChange
    }
}

/// Agent down contribution event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DownContributionEvent {
    /// Time of registering the downed state.
    pub time: u64,

    /// Agent that entered downed state.
    pub agent: AgentId,

    /// Time since last 90% HP in milliseconds.
    pub time_frame: u64,
}

impl Extract for DownContributionEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            time_frame: event.dst_agent,
        }
    }
}

impl TryExtract for DownContributionEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::Last90BeforeDown
    }
}

/// Agent is now an attack target.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttackTargetEvent {
    /// Time of registering the attack target.
    pub time: u64,

    /// Agent that is an attack target.
    pub agent: AgentId,

    /// Parent gadget agent.
    pub parent: AgentId,

    /// Current targetable state.
    pub targetable: i32,
}

impl Extract for AttackTargetEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            parent: AgentId::from_dst(event),
            targetable: event.value,
        }
    }
}

impl TryExtract for AttackTargetEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::AttackTarget
    }
}

/// Agent targetibility change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TargetableEvent {
    /// Time of registering the tergetability change.
    pub time: u64,

    /// Agent that had their targetability changed.
    pub agent: AgentId,

    /// New targetable state.
    pub targetable: i32,
}

impl Extract for TargetableEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            targetable: event.value,
        }
    }
}

impl TryExtract for TargetableEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::Targetable
    }
}
