use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Combat state change kinds.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum StateChange {
    /// Not used, different kind of event.
    None = 0,

    /// Agent entered combat.
    ///
    /// `src_agent` entered combat.
    /// `dst_agent` contains the subgroup.
    /// `value` contains the Profession id.
    /// `buff_dmg` contains the Elite Specialization id.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: yes, limited to squad.
    EnterCombat = 1,

    /// Agent left combat.
    ///
    /// `src_agent` left combat.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: yes, limited to squad.
    ExitCombat = 2,

    /// Agent is now alive.
    ///
    /// `src_agent` is alive.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: yes, limited to squad.
    ChangeUp = 3,

    /// Agent is now dead.
    ///
    /// `src_agent` is dead.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: yes, limited to squad.
    ChangeDead = 4,

    /// Agent is now downed.
    ///
    /// `src_agent` is down.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: yes, limited to squad.
    ChangeDown = 5,

    /// Agent is now in game tracking range.
    ///
    /// `src_agent` is now tracked.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    Spawn = 6,

    /// Source agent is no longer being tracked or out of game tracking range.
    ///
    /// `src_agent` is no longer tracked.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    Despawn = 7,

    /// Agent health change.
    ///
    /// `src_agent` health changed.
    /// `dst_agent` contains percentage as `percent * 10000`.
    /// For example 99.5% will be `9950`.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    HealthUpdate = 8,

    /// Squad combat start, first player entered combat. Logging has started.
    ///
    /// `value` contains the server Unix timestamp as `u32`.
    /// `buff_dmg` contains the local Unix timestamp.
    ///
    /// `src_agent` is `0x637261` (ArcDPS id) if log EVTC and species id if realtime API.
    ///
    /// EVTC: yes
    ///
    /// Realtime: yes
    SquadCombatStart = 9,

    /// Squad combat end, last player has left combat. Logging has ended.
    ///
    /// `value` contains the server Unix timestamp as `u32`.
    /// `buff_dmg` contains the local Unix timestamp.
    ///
    /// `src_agent` is `0x637261` (ArcDPS id) if log EVTC and species id if realtime API.
    ///
    /// EVTC: yes
    ///
    /// Realtime: yes
    SquadCombatEnd = 10,

    /// Agent swapped weapon set.
    ///
    /// `src_agent` swapped weapons.
    /// `dst_agent` contains the new weapon set id.
    /// `value` contains the previous weapon set id.
    ///
    /// `0`/`1` for underwater weapon sets and `4`/`5` for land weapon sets.
    /// `2` is bundle/kit weapon set and `3` transform weapon set.
    ///
    /// EVTC: yes
    ///
    /// Realtime: yes
    WeaponSwap = 11,

    /// Agent maximum health change.
    ///
    /// `src_agent` changed max health.
    /// `dst_agent` contains the new maximum health.
    ///
    /// EVTC: yes, limited to non-players.
    ///
    /// Realtime: no
    MaxHealthUpdate = 12,

    /// Player recording the log.
    ///
    /// `src_agent` is point of view
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    PointOfView = 13,

    /// Game text language.
    ///
    /// `src_agent` contains the text language id.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    Language = 14,

    /// Game build.
    ///
    /// `src_agent` contains the game build.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    GWBuild = 15,

    /// Sever shard id.
    ///
    /// `src_agent` contains the shard id.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    ShardId = 16,

    /// Source agent got a reward chest.
    ///
    /// `src_agent` is always self.
    /// `dst_agent` contains the reward id.
    /// `value` contains the reward type.
    ///
    /// EVTC: yes
    ///
    /// Realtime: yes
    Reward = 17,

    /// Initially present buffs.
    ///
    /// Identical to buff application event.
    /// Appears once per buff per agent on logging start.
    ///
    /// EVTC: yes, limited to squad outside instances.
    ///
    /// Realtime: yes, limited to squad.
    BuffInitial = 18,

    /// Agent position change.
    ///
    /// `src_agent` changed position.
    /// `dst_agent` contains XYZ coordinates as `[f32; 3]`.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    Position = 19,

    /// Agent velocity change.
    ///
    /// `src_agent` changed position.
    /// `dst_agent` contains XYZ velocity as `[f32; 3]`.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    Velocity = 20,

    /// Agent facing change.
    ///
    /// `src_agent` changed position.
    /// `dst_agent` contains XY direction as `[f32; 2]`.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    Facing = 21,

    /// Agent team change.
    ///
    /// `src_agent` changed team.
    /// `dst_agent` contains the new team id.
    /// `value` contains the previous team id.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: yes, limited to squad.
    TeamChange = 22,

    /// Agent is an attack target of parent gadget.
    ///
    /// `src_agent` is the attack target.
    /// `dst_agent` is the parent gadget.
    /// `value` contains the current targetable state.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    AttackTarget = 23,

    /// Agent changed targetable state.
    ///
    /// `src_agent` changed targetable state.
    /// `dst_agent` contains the new targetable state.
    /// `0` for no, `1` for yes. Default is yes.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    Targetable = 24,

    /// Map id.
    ///
    /// `src_agent` contains the map id.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    MapId = 25,

    /// Used internally by ArcDPS.
    /// Should not appear anywhere.
    ReplInfo = 26,

    /// Buff stack is now active.
    ///
    /// `src_agent` has the buff.
    /// `dst_agent` contains the buff stack id marked active.
    /// `value` contains the current buff duration.
    ///
    /// EVTC: yes, limited to squad outside instances.
    ///
    /// Realtime: yes, limited to squad.
    StackActive = 27,

    /// Buff stack duration changed.
    ///
    /// `src_agent` has the buff.
    /// `value` contains the new duration to reset to (also marks inactive).
    /// `pad61-64` contains the stack id.
    ///
    /// EVTC: yes, limited to squad outside instances.
    ///
    /// Realtime: yes, limited to squad.
    StackReset = 28,

    /// Agent is in guild.
    ///
    /// `src_agent` is in guild.
    /// `dst_agent` contains the guild guid as [u8; 16].
    ///
    /// Given in client form, needs minor rearrange for API form.
    ///
    /// EVTC: yes, limited to squad outside instances.
    ///
    /// Realtime: yes, limited to squad.
    Guild = 29,

    /// Buff information.
    ///
    /// `skill_id` is skilldef id of buff.
    /// `is_offcycle` contains the category.
    /// `pad61` contains the stacking type.
    /// `src_master_instance_id` contains the max stacks.
    /// `overstack_value` contains the duration cap.
    ///
    /// If `is_flanking` probably invulnerability-like.
    /// If `is_shields` probably invert-like.
    /// If `pad62` probably resistance-like.
    ///
    /// One event per buff.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    BuffInfo = 30,

    /// Buff formula.
    ///
    /// `skill_id` is skilldef id of buff.
    /// `time` contains `type`, `attr1`, `attr2`, `param1`, `param2`, `param3`, `trait_condition_src` and `trait_condition_self`, `content_reference` as `[f32; 9]`.
    /// `src_instance_id` contains `buff_condition_src` and `buff_condition_self` as `[f32; 2]`.
    ///
    /// If `is_flanking` not NPC.
    /// If `is_shields` not player.
    /// If `is_offcycle` break.
    ///
    /// `overstack_value` is value of type determined by `pad61`.
    ///
    /// One event per buff formula.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    BuffFormula = 31,

    /// Skill information.
    ///
    /// `skill_id` is skilldef id of ability.
    /// `time` contains `recharge`, `range0`, `range1` and `tooltiptime` as `[f32; 4]`.
    ///
    /// One event per ability.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    SkillInfo = 32,

    /// Skill action.
    ///
    /// `skill_id` is skilldef id of ability.
    /// `src_agent` contains the action type.
    /// `dst_agent` contains the time since activation in milliseconds.
    ///
    /// One event per ability timing.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    SkillTiming = 33,

    /// Agent breakbar state change.
    ///
    /// `src_agent` changed breakbar state.
    /// `value` contains the new breakbar state as [`u16`] (game enum: active, recover, immune, none).
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    BreakbarState = 34,

    /// Breakbar percentage.
    ///
    /// `src_agent` has breakbar percentage.
    /// `value` contains percentage as [`f32`].
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    BreakbarPercent = 35,

    /// Message with log integrity information.
    ///
    /// `time` contains the message as a null-terminated C string.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    Integrity = 36,

    /// Agent has a marker.
    ///
    /// `src_agent` has the marker.
    /// `value` contains the markerdef id (volatile, depends on game build).
    /// If `buff`, marker is a commander tag.
    ///
    /// A marker id of `0` indicates a remove.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    Marker = 37,

    /// Agent barrier change.
    ///
    /// `src_agent` has barrier percentage.
    /// `dst_agent` contains percentage as `percent * 10000`.
    /// For example 99.5% will be `9950`.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    BarrierUpdate = 38,

    /// Arc UI stats reset.
    ///
    /// `src_agent` contains the species id of the agent triggering the reset, for example boss species id.
    ///
    /// EVTC: yes
    ///
    /// Realtime: yes
    StatReset = 39,

    /// A custom event created by an extension (addon/plugin).
    Extension = 40,

    /// Delayed combat event.
    ///
    /// Event deemed "unsafe" for realtime that was held back until after squad left combat.
    ///
    /// EVTC: no
    ///
    /// Realtime: yes
    ApiDelayed = 41,

    /// Map instance start timestamp.
    ///
    /// `src_agent` contains the time in milliseconds since the instance was started.
    ///
    /// EVTC: yes
    ///
    /// Realtime: yes
    InstanceStart = 42,

    /// Tick rate health.
    ///
    /// `src_agent` is `25 - tickrate` when `tickrate < 21`.
    /// Every 500ms.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    RateHealth = 43,

    /// Retired since 230716.
    ///
    /// Previously: *Last 90% before down.*
    ///
    /// *`src_agent` is enemy agent that went down, `dst_agent` is time in ms since last 90%.
    /// For downs contribution.*
    Last90BeforeDown = 44,

    /// Retired since 230716.
    ///
    /// Previously: *Effect created or ended.*
    ///
    /// *`skill_id` contains the effect id.*
    /// *`src_agent` is the effect owner.*
    /// *`dst_agent` if effect located at agent.*
    /// *Otherwise `value` contains XYZ position as `[f32; 3]`, `affinity` contains XY orientation as `[f32; 2]`, `pad61` contains Z orientation as [`f32`].*
    /// *`is_shields` contains duration as [`u16`].*
    /// *If `is_flanking`, duration is a tracking id.*
    /// *If effect id is `0`, effect ended and `is_shields` contains tracking id.*
    EffectOld = 45,

    /// Content id to GUID.
    ///
    /// `skill_id` is the content id.
    /// `src_agent` contains the persistent content guid as `[u8; 16]`.
    /// `overstack_value` contains a variant of [`ContentLocal`](crate::guid::ContentLocal).
    ///
    /// *Not used in realtime API.*
    IdToGUID = 46,

    /// Log boss agent changed.
    ///
    /// `src_agent` contains the species id of the agent.
    /// `dst_agent` is the boss agent.
    /// `value` contains the server Unix timestamp as `u32`.
    /// `buff_dmg` contains the local Unix timestamp.
    ///
    /// EVTC: yes
    ///
    /// Realtime: yes
    LogNPCUpdate = 47,

    /// Used internally by ArcDPS.
    /// Should not appear anywhere.
    IdleEvent = 48,

    /// A custom combat event created by an extension (addon/plugin).
    ///
    /// `skill_id` is treated as skill id and will be added to the EVTC skill table.
    ExtensionCombat = 49,

    /// Fractal scale.
    ///
    /// `src_agent` contains the scale.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    FractalScale = 50,

    /// Visual effect created or ended.
    ///
    /// `skill_id` contains the effect id.
    /// `src_agent` is the effect owner (if any).
    /// `dst_agent` if effect located at agent.
    /// Otherwise `value` contains XYZ location as `[f32; 3]`.
    /// `affinity` contains duration as [`u32`].
    /// `is_buffremove` contains trackable id as [`u32`].
    /// `is_shields` contains orientation as `[i16; 3]`.
    /// Orientation values are `original * 1000` or [`i16::MIN`]/[`i16::MAX`] if out of bounds.
    ///
    /// EVTC: yes, limited to agent table outside instances
    ///
    /// Realtime: no
    Effect = 51,

    /// Combat ruleset.
    ///
    /// `src_agent` has bit 0 set if PvE rules, bit 1 if WvW rules and bit 2 if PvP rules.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    Ruleset = 52,

    /// Squad ground marker placed or removed.
    ///
    /// `src_agent` contains XYZ location as `[f32; 3]` or [`f32::INFINITY`] if removed.
    /// `skill_id` contains the index of the squad marker, for example `0` for Arrow marker.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    SquadMarker = 53,

    /// ArcDPS build information.
    ///
    /// `src_agent` contains ArcDPS build as null-terminated C string.
    ///
    /// EVTC: yes
    ///
    /// Realtime: no
    ArcBuild = 54,

    /// Agent gliding state change.
    ///
    /// `src_agent` changed gliding state.
    /// `value` contains `1` if deployed and `0` if stowed.
    ///
    /// EVTC: yes, limited to agent table outside instances.
    ///
    /// Realtime: no
    Glider = 55,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl StateChange {
    /// Checks whether the statechange has an associated timestamp.
    #[inline]
    pub fn has_time(&self) -> bool {
        matches!(
            self,
            Self::None
                | Self::EnterCombat
                | Self::ExitCombat
                | Self::ChangeUp
                | Self::ChangeDead
                | Self::ChangeDown
                | Self::Spawn
                | Self::Despawn
                | Self::HealthUpdate
                | Self::SquadCombatStart
                | Self::SquadCombatEnd
                | Self::WeaponSwap
                | Self::MaxHealthUpdate
                | Self::Reward
                | Self::BuffInitial
                | Self::Position
                | Self::Velocity
                | Self::Facing
                | Self::TeamChange
                | Self::AttackTarget
                | Self::Targetable
                | Self::StackActive
                | Self::StackReset
                | Self::BreakbarState
                | Self::BreakbarPercent
                | Self::BarrierUpdate
                | Self::StatReset
                | Self::Extension
                | Self::ApiDelayed
                | Self::Last90BeforeDown
                | Self::EffectOld
                | Self::LogNPCUpdate
                | Self::ExtensionCombat
                | Self::Effect
                | Self::SquadMarker
                | Self::Glider
        )
    }
}
