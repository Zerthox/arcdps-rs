use crate::{
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
    Event, EventCategory, Language, StateChange,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Possible [`Event`] kinds.
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
    WeaponSwap(WeaponSwapEvent),

    /// Agent maximum health change.
    MaxHealthUpdate(MaxHealthEvent),

    /// Agent is "recording" player.
    PointOfView(AgentStatusEvent),

    /// Game text language.
    Language {
        time: u64,
        language: Result<Language, u64>,
    },

    /// Game build.
    GWBuild { time: u64, build: u64 },

    /// Sever shard id.
    ShardId { time: u64, shard: u64 },

    /// Agent got a reward chest.
    Reward(RewardEvent),

    /// Appears once per buff per agent on logging start.
    BuffInitial(BuffApplyEvent),

    /// Agent position change.
    Position(PositionEvent),

    /// Agent velocity change.
    Velocity(PositionEvent),

    /// Agent facing change.
    Facing(PositionEvent),

    /// Agent team change.
    TeamChange(TeamChangeEvent),

    /// Agent is now an attack target.
    AttackTarget(AttackTargetEvent),

    /// Agent targetability change.
    Targetable(TargetableEvent),

    /// Map id.
    MapId { time: u64, map: u64 },

    /// Agent with active buff.
    StackActive(StackActiveEvent),

    /// Agent with reset buff.
    StackReset(StackResetEvent),

    /// Agent is in guild.
    Guild(GuildEvent),

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
    Error(ErrorEvent),

    /// Agent has tag.
    Tag(TagEvent),

    /// Agent barrier change.
    BarrierUpdate(BarrierUpdateEvent),

    /// Arc UI stats reset.
    StatReset { time: u64, target: u64 },

    /// A custom event created by an extension (addon/plugin).
    Extension(Event),

    /// Delayed combat event.
    ApiDelayed(Box<EventKind>),

    /// Instance started.
    InstanceStart { time: u64, start: u64 },

    /// Tick rate.
    Tickrate { time: u64, rate: u64 },

    /// Last 90% before down for downs contribution..
    Last90BeforeDown(DownContributionEvent),

    /// Effect created or ended.
    EffectOld(EffectOld),

    /// Id to GUID.
    IdToGUID(EffectGUID),

    /// Log NPC changed.
    LogNPCUpdate(LogEvent),

    /// A custom combat event created by an extension (addon/plugin).
    ExtensionCombat(Event),

    /// Fractal scale.
    FractalScale { time: u64, scale: u64 },

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
    Unknown(Event),
}

impl From<Event> for EventKind {
    #[inline]
    fn from(mut event: Event) -> Self {
        unsafe {
            match event.categorize() {
                EventCategory::StateChange => match event.get_statechange() {
                    StateChange::None => unreachable!("statechange none in statechange category"),
                    StateChange::IdleEvent | StateChange::ReplInfo => {
                        unreachable!("illegal internal statechange")
                    }
                    StateChange::EnterCombat => Self::EnterCombat(event.extract()),
                    StateChange::ExitCombat => Self::ExitCombat(event.extract()),
                    StateChange::ChangeUp => Self::ChangeUp(event.extract()),
                    StateChange::ChangeDead => Self::ChangeDead(event.extract()),
                    StateChange::ChangeDown => Self::ChangeDown(event.extract()),
                    StateChange::Spawn => Self::Spawn(event.extract()),
                    StateChange::Despawn => Self::Despawn(event.extract()),
                    StateChange::HealthUpdate => Self::HealthUpdate(event.extract()),
                    StateChange::LogStart => Self::LogStart(event.extract()),
                    StateChange::LogEnd => Self::LogEnd(event.extract()),
                    StateChange::WeaponSwap => Self::WeaponSwap(event.extract()),
                    StateChange::MaxHealthUpdate => Self::MaxHealthUpdate(event.extract()),
                    StateChange::PointOfView => Self::PointOfView(event.extract()),
                    StateChange::Language => Self::Language {
                        time: event.time,
                        language: Language::try_from(event.src_agent as i32)
                            .map_err(|_| event.src_agent),
                    },
                    StateChange::GWBuild => Self::GWBuild {
                        time: event.time,
                        build: event.src_agent,
                    },
                    StateChange::ShardId => Self::ShardId {
                        time: event.time,
                        shard: event.src_agent,
                    },
                    StateChange::Reward => Self::Reward(event.extract()),
                    StateChange::BuffInitial => Self::BuffInitial(event.extract()),
                    StateChange::Position => Self::Position(event.extract()),
                    StateChange::Velocity => Self::Velocity(event.extract()),
                    StateChange::Facing => Self::Facing(event.extract()),
                    StateChange::TeamChange => Self::TeamChange(event.extract()),
                    StateChange::AttackTarget => Self::AttackTarget(event.extract()),
                    StateChange::Targetable => Self::Targetable(event.extract()),
                    StateChange::MapId => Self::MapId {
                        time: event.time,
                        map: event.src_agent,
                    },
                    StateChange::StackActive => Self::StackActive(event.extract()),
                    StateChange::StackReset => Self::StackReset(event.extract()),
                    StateChange::Guild => Self::Guild(event.extract()),
                    StateChange::BuffInfo => Self::BuffInfo(event.extract()),
                    StateChange::BuffFormula => Self::BuffFormula(event.extract()),
                    StateChange::SkillInfo => Self::SkillInfo(event.extract()),
                    StateChange::SkillTiming => Self::SkillTiming(event.extract()),
                    StateChange::BreakbarState => Self::BreakbarState(event.extract()),
                    StateChange::BreakbarPercent => Self::BreakbarPercent(event.extract()),
                    StateChange::Error => Self::Error(event.extract()),
                    StateChange::Tag => Self::Tag(event.extract()),
                    StateChange::BarrierUpdate => Self::BarrierUpdate(event.extract()),
                    StateChange::StatReset => Self::StatReset {
                        time: event.time,
                        target: event.src_agent,
                    },
                    StateChange::Extension => Self::Extension(event),
                    StateChange::ApiDelayed => {
                        event.is_statechange = StateChange::None.into();
                        Self::ApiDelayed(event.into_kind().into())
                    }
                    StateChange::InstanceStart => Self::InstanceStart {
                        time: event.time,
                        start: event.src_agent,
                    },
                    StateChange::Tickrate => Self::Tickrate {
                        time: event.time,
                        rate: event.src_agent,
                    },
                    StateChange::Last90BeforeDown => Self::Last90BeforeDown(event.extract()),
                    StateChange::EffectOld => Self::EffectOld(event.extract()),
                    StateChange::IdToGUID => Self::IdToGUID(event.extract()),
                    StateChange::LogNPCUpdate => Self::LogNPCUpdate(event.extract()),
                    StateChange::ExtensionCombat => Self::ExtensionCombat(event),
                    StateChange::FractalScale => Self::FractalScale {
                        time: event.time,
                        scale: event.src_agent,
                    },
                    StateChange::Effect => Self::Effect(event.extract()),
                    StateChange::Unknown(_) => Self::Unknown(event),
                },
                EventCategory::Activation => Self::Activation(event.extract()),
                EventCategory::BuffRemove => Self::Activation(event.extract()),
                EventCategory::BuffApply => Self::Activation(event.extract()),
                EventCategory::BuffDamage => Self::Activation(event.extract()),
                EventCategory::Strike => Self::Strike(event.extract()),
            }
        }
    }
}
