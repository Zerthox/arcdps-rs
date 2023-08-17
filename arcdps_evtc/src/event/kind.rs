use crate::{
    agent::{
        AgentStatusEvent, BarrierUpdateEvent, EnterCombatEvent, HealthUpdateEvent, MaxHealthEvent,
    },
    breakbar::{BreakbarPercentEvent, BreakbarStateEvent},
    buff::{BuffApplyEvent, BuffDamageEvent, BuffFormula, BuffInfo, BuffRemoveEvent},
    effect::{Effect, EffectGUID, EffectOld},
    log::LogEvent,
    position::PositionEvent,
    skill::{ActivationEvent, SkillInfo, SkillTiming},
    strike::StrikeEvent,
    AgentId, CombatEvent, Language,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Possible [`CombatEvent`] kinds.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EventKind {
    /// Agent entered combat.
    EnterCombat(EnterCombatEvent),

    /// Agent left combat.
    ExitCombat(AgentStatusEvent),

    /// Agent is now alive.
    ChangeUp(AgentStatusEvent),

    /// Agent is now dead.
    ChangeDead(AgentStatusEvent),

    /// Agent is now downed.
    ChangeDown(AgentStatusEvent),

    /// Agent is now in game tracking range.
    Spawn(AgentStatusEvent),

    /// Agent is no longer being tracked or out of game tracking range.
    Despawn(AgentStatusEvent),

    /// Agent health change.
    HealthUpdate(HealthUpdateEvent),

    /// Log started.
    LogStart(LogEvent),

    /// Log ended.
    LogEnd(LogEvent),

    /// Agent swapped weapon set.
    WeaponSwap(AgentStatusEvent),

    /// Agent maximum health change.
    MaxHealthUpdate(MaxHealthEvent),

    /// Agent is "recording" player.
    PointOfView(AgentStatusEvent),

    /// Game text language.
    Language(Language),

    /// Game build.
    GWBuild(u64),

    /// Sever shard id.
    ShardId(u64),

    /// Agent got a reward chest.
    Reward(AgentStatusEvent),

    /// Appears once per buff per agent on logging start.
    BuffInitial(BuffApplyEvent),

    /// Agent position change.
    Position(PositionEvent),

    /// Agent velocity change.
    Velocity(PositionEvent),

    /// Agent facing change.
    Facing(PositionEvent),

    /// Agent team change.
    TeamChange(AgentStatusEvent),

    /// Agent is now an attack target.
    AttackTarget {
        time: u64,
        agent: AgentId,
        parent_agent: AgentId,
        targetable_state: i32,
    },

    /// Agent targetability change.
    Targetable {
        time: u64,
        agent: AgentId,
        targetable: bool,
    },

    /// Map id.
    MapId(u64),

    /// Agent with active buff.
    StackActive {
        time: u64,
        agent: AgentId,
        stack_id: u64,
    },

    /// Agent with reset buff.
    StackReset {
        time: u64,
        agent: AgentId,
        duration: i32,
        stack_id: u64,
    },

    /// Agent is in guild.
    Guild { agent: AgentId, guild: u128 },

    /// Buff information.
    BuffInfo(BuffInfo),

    /// Buff formula.
    BuffFormula(BuffFormula),

    /// Skill information.
    SkillInfo(SkillInfo),

    /// Skill action.
    SkillTiming(SkillTiming),

    /// Agent breakbar state change.
    BreakbarState(BreakbarStateEvent),

    /// Breakbar percentage.
    BreakbarPercent(BreakbarPercentEvent),

    /// Error.
    Error(String),

    /// Agent has tag.
    Tag { agent: AgentId, tag: u32 },

    /// Agent barrier change.
    BarrierUpdate(BarrierUpdateEvent),

    /// Arc UI stats reset.
    StatReset { target: u64 },

    /// A custom event created by an extension (addon/plugin).
    Extension(CombatEvent),

    /// Delayed combat event.
    ApiDelayed(CombatEvent),

    /// Instance started.
    InstanceStart(u64),

    /// Tick rate.
    Tickrate(u64),

    /// Last 90% before down.
    Last90BeforeDown(),

    /// Effect created or ended.
    EffectOld(EffectOld),

    /// Id to GUID.
    IdToGUID(EffectGUID),

    /// Log NPC changed.
    LogNPCUpdate(LogEvent),

    /// A custom combat event created by an extension (addon/plugin).
    ExtensionCombat(CombatEvent),

    /// Fractal scale.
    FractalScale(u64),

    /// Effect created or ended.
    Effect(Effect),

    /// Activation (cast) event.
    Activation(ActivationEvent),

    /// Buff removed.
    BuffRemove(BuffRemoveEvent),

    /// Buff applied.
    BuffApply(BuffApplyEvent),

    /// Buff damage.
    BuffDamage(BuffDamageEvent),

    /// Direct (strike) damage.
    Strike(StrikeEvent),

    /// Unknown event.
    Unknown(CombatEvent),
}

impl From<CombatEvent> for EventKind {
    #[inline]
    fn from(event: CombatEvent) -> Self {
        // TODO: conversions
        Self::Unknown(event)
    }
}
