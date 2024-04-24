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

    /// Source agent entered combat.
    ///
    /// `dst_agent` contains the subgroup.
    EnterCombat = 1,

    /// Source agent left combat.
    ExitCombat = 2,

    /// Source agent is now alive.
    ChangeUp = 3,

    /// Source agent is now dead.
    ChangeDead = 4,

    /// Source agent is now downed.
    ChangeDown = 5,

    /// Source agent is now in game tracking range.
    ///
    /// *Not used in realtime API.*
    Spawn = 6,

    /// Source agent is no longer being tracked or out of game tracking range.
    ///
    /// *Not used in realtime API.*
    Despawn = 7,

    /// Source agent health change.
    ///
    /// `dst_agent` contains percentage as `percent * 10000`.
    /// For example 99.5% will be `9950`.
    ///
    /// *Not used in realtime API.*
    HealthUpdate = 8,

    /// Logging has started.
    ///
    /// `value` contains the server Unix timestamp as `u32`.
    /// `buff_dmg` contains the local Unix timestamp.
    ///
    /// `src_agent` is `0x637261` (ArcDPS id) if log EVTC and species id if realtime API.
    LogStart = 9,

    /// Logging has ended.
    ///
    /// `value` contains the server Unix timestamp as `u32`.
    /// `buff_dmg` contains the local Unix timestamp.
    ///
    /// `src_agent` is `0x637261` (ArcDPS id) if log EVTC and species id if realtime API.
    LogEnd = 10,

    /// Source agent swapped weapon set.
    ///
    /// `dst_agent` contains the current set id.
    /// `0`/`1` for underwater weapon sets and `4`/`5` for land weapon sets.
    /// `2` is bundle/kit weapon set and `3` transform weapon set.
    WeaponSwap = 11,

    /// Source agent maximum health change.
    ///
    /// `dst_agent` contains the new maximum health.
    ///
    /// *Not used in realtime API.*
    MaxHealthUpdate = 12,

    /// Source agent is "recording" player.
    ///
    /// *Not used in realtime API.*
    PointOfView = 13,

    /// Source agent contains the game text language.
    ///
    /// *Not used in realtime API.*
    Language = 14,

    /// Source agent contains the game build.
    ///
    /// *Not used in realtime API.*
    GWBuild = 15,

    /// Source agent contains the sever shard id.
    ///
    /// *Not used in realtime API.*
    ShardId = 16,

    /// Source agent got a reward chest.
    ///
    /// Source is always self.
    /// `dst_agent` contains the reward id.
    /// Value contains the reward type.
    Reward = 17,

    /// Appears once per buff per agent on logging start.
    ///
    /// *(`statechange == 18` and `buff == 18`, normal combat event otherwise)*
    BuffInitial = 18,

    /// Source agent position change.
    ///
    /// `dst_agent` contains x/y/z as array of 3 floats.
    ///
    /// *Not used in realtime API.*
    Position = 19,

    /// Source agent velocity change.
    ///
    /// `dst_agent` contains x/y/z as array of 3 floats.
    ///
    /// *Not used in realtime API.*
    Velocity = 20,

    /// Source agent facing change.
    ///
    /// `dst_agent` contains x/y as array of 2 floats.
    ///
    /// *Not used in realtime API.*
    Facing = 21,

    /// Source agent team change.
    ///
    /// `dst_agent` contains the new team id.
    TeamChange = 22,

    /// Source agent is now an attack target.
    ///
    /// `dst_agent` is the parent agent (gadget type).
    /// `value` contains the current targetable state.
    ///
    /// *Not used in realtime API.*
    AttackTarget = 23,

    /// Source agent targetability change.
    ///
    /// `dst_agent` contains the new targetable state.
    /// `0` for no, `1` for yes. Default is yes.
    ///
    /// *Not used in realtime API.*
    Targetable = 24,

    /// Source agent contains the map id.
    ///
    /// *Not used in realtime API.*
    MapId = 25,

    /// Used internally by ArcDPS.
    /// Should not appear anywhere.
    ReplInfo = 26,

    /// Source agent with active buff.
    ///
    /// `dst_agent` contains the stack id marked active.
    StackActive = 27,

    /// Source agent with reset buff.
    ///
    /// `value` is the duration to reset to (also marks inactive).
    /// `pad61` contains the stack id.
    StackReset = 28,

    /// Source agent is in guild.
    ///
    /// `dst_agent` until `buff_dmg` is [`u128`] (16 byte) guid.
    ///
    /// Given in client form, needs minor rearrange for API form.
    Guild = 29,

    /// Buff information.
    ///
    /// `is_offcycle` contains the category.
    /// `pad61` contains the stacking type.
    /// `src_master_instance_id` contains the max stacks.
    /// `overstack_value` contains the duration cap.
    ///
    /// If `is_flanking` probably invulnerable.
    /// If `is_shields` probably invert.
    /// If `pad62` probably resistance.
    ///
    /// *Not used in realtime API.*
    BuffInfo = 30,

    /// Buff formula.
    ///
    /// `time` contains `type`, `attr1`, `attr2`, `param1`, `param2`, `param3`, `trait_src` and `trait_self` as `[f32; 8]`.
    /// `src_instance_id` contains `buff_src` and `buff_self` as `[f32; 2]`.
    ///
    /// If `is_flanking` not NPC.
    /// If `is_shields` not player.
    /// If `is_offcycle` break.
    ///
    /// `overstack_value` is value of type determined by `pad61`.
    ///
    /// Once per formula.
    ///
    /// *Not used in realtime API.*
    BuffFormula = 31,

    /// Skill information.
    ///
    /// `time` contains `recharge`, `range0`, `range1` and `tooltiptime` as `[f32; 4]`.
    ///
    /// *Not used in realtime API.*
    SkillInfo = 32,

    /// Skill action.
    ///
    /// `src_agent` contains the action.
    /// `dst_agent` contains at which millisecond.
    ///
    /// One per timing.
    ///
    /// *Not used in realtime API.*
    SkillTiming = 33,

    /// Source agent breakbar state change.
    ///
    /// Value is [`u16`] game enum (active, recover, immune, none).
    ///
    /// *Not used in realtime API.*
    BreakbarState = 34,

    /// Breakbar percentage.
    ///
    /// `value` contains percentage as float.
    ///
    /// *Not used in realtime API.*
    BreakbarPercent = 35,

    /// Error.
    ///
    /// `time` contains the error message as an array of up to 32 characters.
    ///
    /// *Not used in realtime API.*
    Error = 36,

    /// Source agent has marker.
    ///
    /// `src_agent` is agent.
    /// `value` is the id of the marker (volatile, depends on game build).
    /// `buff` will be non-zero if commander.
    ///
    /// A marker id of `0` indicates a remove.
    Marker = 37,

    /// Source agent barrier change.
    ///
    /// `dst_agent` contains percentage as `percent * 10000`.
    /// For example 99.5% will be `9950`.
    ///
    /// *Not used in realtime API.*
    BarrierUpdate = 38,

    /// Arc UI stats reset.
    ///
    /// `src_agent` contains the NPC id of the active log.
    ///
    /// *Not used in log EVTC.*
    StatReset = 39,

    /// A custom event created by an extension (addon/plugin).
    Extension = 40,

    /// Delayed combat event.
    ApiDelayed = 41,

    /// Instance started.
    ///
    /// `src_agent` contains the time in ms at which the instance was likely started.
    InstanceStart = 42,

    /// Tick rate.
    ///
    /// Every 500ms.
    /// `src_agent` is `25 - tickrate` (when `tickrate < 21`).
    Tickrate = 43,

    /// Last 90% before down.
    ///
    /// `src_agent` is enemy agent that went down, `dst_agent` is time in ms since last 90%.
    /// For downs contribution.
    Last90BeforeDown = 44,

    /// Effect created or ended.
    ///
    /// `skill_id` contains the effect id.
    /// `src_agent` is the effect owner.
    /// `dst_agent` if effect located at agent.
    /// Otherwise `value` contains XYZ position as `[f32; 3]`, `affinity` contains XY orientation as `[f32; 2]`, `pad61` contains Z orientation as [`f32`].
    /// `is_shields` contains duration as [`u16`].
    /// If `is_flanking`, duration is a tracking id.
    /// If effect id is `0`, effect ended and `is_shields` contains tracking id.
    ///
    /// *Not used in realtime API.*
    EffectOld = 45,

    /// Id to GUID.
    ///
    /// `src_agent` contains [`u128`] (16 byte) persistent content guid.
    /// `overstack_value` is a variant of [`ContentLocal`](crate::guid::ContentLocal), `skill_id` is content id.
    ///
    /// *Not used in realtime API.*
    IdToGUID = 46,

    /// Log NPC changed.
    ///
    /// `value` contains the server Unix timestamp as `u32`.
    /// `buff_dmg` contains the local Unix timestamp.
    ///
    /// `src_agent` is species id.
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
    FractalScale = 50,

    /// Effect created or ended.
    ///
    /// `skill_id` contains the effect id.
    /// `src_agent` is the effect owner.
    /// `dst_agent` if effect located at agent.
    /// Otherwise `value` contains XYZ position as `[f32; 3]`.
    /// `affinity` contains duration as [`u32`].
    /// `is_buffremove` contains trackable id as [`u32`].
    /// `is_shields` contains orientation as `[i16; 3]`.
    /// Orientation values are original multiplied by `1000` or [`i16::MIN`]/[`i16::MAX`] if out of bounds.
    ///
    /// *Not used in realtime API.*
    Effect = 51,

    /// Combat ruleset.
    ///
    /// `src_agent` has bit 0 set if PvE rules buff, bit 1 if WvW rules and bit 2 if PvP rules.
    Ruleset = 52,

    /// Squad marker placed or removed.
    ///
    /// `src_agent` contains the XYZ location as `[f32; 3]` or [`f32::INFINITY`] if removed.
    /// `skill_id` contains the index of the squad marker.
    SquadMarker = 53,

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
                | Self::LogStart
                | Self::LogEnd
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
        )
    }
}
