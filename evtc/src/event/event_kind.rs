use crate::{
    agent::{
        AgentStatusEvent, AttackTargetEvent, BarrierUpdateEvent, BreakbarPercentEvent,
        BreakbarStateEvent, DownContributionEvent, EnterCombatEvent, GliderEvent,
        HealthUpdateEvent, MaxHealthEvent, StunbreakEvent, TargetableEvent, TeamChangeEvent,
    },
    buff::{
        BuffApplyEvent, BuffDamageEvent, BuffFormula, BuffInfo, BuffInitialEvent, BuffRemoveEvent,
        StackActiveEvent, StackResetEvent,
    },
    content::ContentInfo,
    effect::{
        AgentEffect, AgentEffectRemove, Effect45, Effect51, GroundEffect, GroundEffectRemove,
    },
    log::{ArcBuildEvent, ErrorEvent, LogEvent},
    marker::{AgentMarkerEvent, SquadMarkerEvent},
    missile::{MissileCreate, MissileLaunch, MissileRemove},
    player::{GuildEvent, RewardEvent},
    position::PositionEvent,
    ruleset::Ruleset,
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
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
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
    SquadCombatStart(LogEvent),

    /// Log ended.
    SquadCombatEnd(LogEvent),

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
    BuffInitial(BuffInitialEvent),

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
    Integrity(ErrorEvent),

    /// Agent has marker.
    AgentMarker(AgentMarkerEvent),

    /// Agent barrier change.
    BarrierUpdate(BarrierUpdateEvent),

    /// Arc UI stats reset.
    StatReset { time: u64, target: u64 },

    /// A custom event created by an extension (addon/plugin).
    Extension { sig: u32, event: Event },

    /// Delayed combat event.
    ApiDelayed { event: Box<EventKind> },

    /// Instance started.
    InstanceStart { time: u64, start: u64 },

    /// Tick rate.
    RateHealth { time: u64, rate: u64 },

    /// Last 90% before down for downs contribution.
    Last90BeforeDown(DownContributionEvent),

    /// Effect created or ended.
    Effect45(Effect45),

    /// Content id to GUID.
    ///
    /// This maps a volatile content id to a stable GUID.
    IdToGUID(ContentInfo),

    /// Log NPC changed.
    LogNPCUpdate(LogEvent),

    /// A custom combat event created by an extension (addon/plugin).
    ExtensionCombat { sig: u32, event: Event },

    /// Fractal scale.
    FractalScale { time: u64, scale: u64 },

    /// Effect created or ended.
    Effect51(Effect51),

    /// Combat ruleset.
    Ruleset(Ruleset),

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

    /// Squad marker placed or removed.
    SquadMarker(SquadMarkerEvent),

    /// ArcDPS build information.
    ArcBuild(ArcBuildEvent),

    /// Agent gliding state changed.
    Glider(GliderEvent),

    /// Effect
    Stunbreak(StunbreakEvent),

    /// Missile created.
    MissileCreate(MissileCreate),

    /// Missile launched or relaunched.
    MissileLaunch(MissileLaunch),

    /// Missile removed or destroyed.
    MissileRemove(MissileRemove),

    /// Ground effect created.
    EffectGroundCreate(GroundEffect),

    /// Ground effect removed.
    EffectGroundRemove(GroundEffectRemove),

    /// Effect around Agent created.
    EffectAgentCreate(AgentEffect),

    /// Effect around Agent removed.
    EffectAgentRemove(AgentEffectRemove),

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
                    StateChange::SquadCombatStart => Self::SquadCombatStart(event.extract()),
                    StateChange::SquadCombatEnd => Self::SquadCombatEnd(event.extract()),
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
                    StateChange::Integrity => Self::Integrity(event.extract()),
                    StateChange::Marker => Self::AgentMarker(event.extract()),
                    StateChange::BarrierUpdate => Self::BarrierUpdate(event.extract()),
                    StateChange::StatReset => Self::StatReset {
                        time: event.time,
                        target: event.src_agent,
                    },
                    StateChange::Extension => Self::Extension {
                        sig: event.get_pad_id(),
                        event,
                    },
                    StateChange::ApiDelayed => {
                        event.is_statechange = StateChange::None.into();
                        Self::ApiDelayed {
                            event: event.into_kind().into(),
                        }
                    }
                    StateChange::InstanceStart => Self::InstanceStart {
                        time: event.time,
                        start: event.src_agent,
                    },
                    StateChange::RateHealth => Self::RateHealth {
                        time: event.time,
                        rate: event.src_agent,
                    },
                    StateChange::Last90BeforeDown => Self::Last90BeforeDown(event.extract()),
                    StateChange::Effect45 => Self::Effect45(event.extract()),
                    StateChange::IdToGUID => Self::IdToGUID(event.extract()),
                    StateChange::LogNPCUpdate => Self::LogNPCUpdate(event.extract()),
                    StateChange::ExtensionCombat => Self::ExtensionCombat {
                        sig: event.get_pad_id(),
                        event,
                    },
                    StateChange::FractalScale => Self::FractalScale {
                        time: event.time,
                        scale: event.src_agent,
                    },
                    StateChange::Effect51 => Self::Effect51(event.extract()),
                    StateChange::Ruleset => {
                        Self::Ruleset(Ruleset::from_bits_retain(event.src_agent))
                    }
                    StateChange::SquadMarker => Self::SquadMarker(event.extract()),
                    StateChange::ArcBuild => Self::ArcBuild(event.extract()),
                    StateChange::Glider => Self::Glider(event.extract()),
                    StateChange::Stunbreak => Self::Stunbreak(event.extract()),
                    StateChange::MissileCreate => Self::MissileCreate(event.extract()),
                    StateChange::MissileLaunch => Self::MissileLaunch(event.extract()),
                    StateChange::MissileRemove => Self::MissileRemove(event.extract()),
                    StateChange::EffectGroundCreate => Self::EffectGroundCreate(event.extract()),
                    StateChange::EffectGroundRemove => Self::EffectGroundRemove(event.extract()),
                    StateChange::EffectAgentCreate => Self::EffectAgentCreate(event.extract()),
                    StateChange::EffectAgentRemove => Self::EffectAgentRemove(event.extract()),
                    StateChange::Unknown(_) => Self::Unknown(event),
                },
                EventCategory::Activation => Self::Activation(event.extract()),
                EventCategory::BuffRemove => Self::BuffRemove(event.extract()),
                EventCategory::BuffApply => Self::BuffApply(event.extract()),
                EventCategory::BuffDamage => Self::BuffDamage(event.extract()),
                EventCategory::Strike => Self::Strike(event.extract()),
            }
        }
    }
}
