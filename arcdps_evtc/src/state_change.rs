use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

// TODO: non-exhaustive instead of conversion from primitive?

/// Combat state change.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum StateChange {
    /// Not used, different kind of event.
    None,

    /// Source agent entered combat.
    ///
    /// `dst_agent` contains the subgroup.
    EnterCombat,

    /// Source agent left combat.
    ExitCombat,

    /// Source agent is now alive.
    ChangeUp,

    /// Source agent is now dead.
    ChangeDead,

    /// Source agent is now downed.
    ChangeDown,

    /// Source agent is now in game tracking range.
    ///
    /// *Not used in realtime API.*
    Spawn,

    /// Source agent is no longer being tracked or out of game tracking range.
    ///
    /// *Not used in realtime API.*
    Despawn,

    /// Source agent health change.
    ///
    /// `dst_agent` contains percentage as `percent * 10000`.
    /// For example 99.5% will be `9950`.
    ///
    /// *Not used in realtime API.*
    HealthUpdate,

    /// Logging has started.
    ///
    /// `value` contains the server Unix timestamp as `u32`.
    /// `buff_dmg` contains the local Unix timestamp.
    ///
    /// `arc_agent` is `0x637261` (ArcDPS id) if log EVTC and species id if realtime API.
    LogStart,

    /// Logging has ended.
    ///
    /// `value` contains the server Unix timestamp as `u32`.
    /// `buff_dmg` contains the local Unix timestamp.
    ///
    /// `src_agent` is `0x637261` (ArcDPS id) if log EVTC and species id if realtime API.
    LogEnd,

    /// Source agent swapped weapon set.
    ///
    /// `dst_agent` contains the current set id.
    /// `0`/`1` for underwater weapon sets and `4`/`5` for land weapon sets.
    /// `2` is bundle/kit weapon set and `3` transform weapon set.
    WeaponSwap,

    /// Source agent maximum health change.
    ///
    /// `dst_agent` contains the new maximum health.
    ///
    /// *Not used in realtime API.*
    MaxHealthUpdate,

    /// Source agent is "recording" player.
    ///
    /// *Not used in realtime API.*
    PointOfView,

    /// Source agent contains the game text language.
    ///
    /// *Not used in realtime API.*
    Language,

    /// Source agent contains the game build.
    ///
    /// *Not used in realtime API.*
    GWBuild,

    /// Source agent contains the sever shard id.
    ///
    /// *Not used in realtime API.*
    ShardId,

    /// Source agent got a reward chest.
    ///
    /// Source is always self.
    /// `dst_agent` contains the reward id.
    /// Value contains the reward type.
    Reward,

    /// Appears once per buff per agent on logging start.
    ///
    /// *(`statechange == 18` and `buff == 18`, normal combat event otherwise)*
    BuffInitial,

    /// Source agent position change.
    ///
    /// `dst_agent` contains x/y/z as array of 3 floats.
    ///
    /// *Not used in realtime API.*
    Position,

    /// Source agent velocity change.
    ///
    /// `dst_agent` contains x/y/z as array of 3 floats.
    ///
    /// *Not used in realtime API.*
    Velocity,

    /// Source agent facing change.
    ///
    /// `dst_agent` contains x/y as array of 2 floats.
    ///
    /// *Not used in realtime API.*
    Facing,

    /// Source agent team change.
    ///
    /// `dst_agent` contains the new team id.
    TeamChange,

    /// Source agent is now an attack target.
    ///
    /// `dst_agent` is the parent agent (gadget type).
    /// `value` contains the current targetable state.
    ///
    /// *Not used in realtime API.*
    AttackTarget,

    /// Source agent targetability change.
    ///
    /// `dst_agent` contains the new targetable state.
    /// `0` for no, `1` for yes. Default is yes.
    ///
    /// *Not used in realtime API.*
    Targetable,

    /// Source agent contains the map id.
    ///
    /// *Not used in realtime API.*
    MapId,

    /// Used internally by ArcDPS.
    /// Should not appear anywhere.
    ReplInfo,

    /// Source agent with active buff.
    ///
    /// `dst_agent` contains the stack id marked active.
    StackActive,

    /// Source agent with reset buff.
    ///
    /// `value` is the duration to reset to (also marks inactive).
    /// `pad61` contains the stack id.
    StackReset,

    /// Source agent is in guild.
    ///
    /// `dst_agent` until `buff_dmg` is [`u128`] (16 byte) guid.
    ///
    /// Given in client form, needs minor rearrange for API form.
    Guild,

    /// Buff information.
    ///
    /// If `is_flanking` probably invulnerable.
    /// If `is_shields` probably invert.
    ///
    /// `is_off_cycle` contains the category.
    /// `pad61` contains the stacking type.
    /// `pad62` contains the probably resistance.
    /// `src_master_instid` contains the max stacks.
    /// `overstack_value` contains the duration cap.
    ///
    /// *Not used in realtime API.*
    BuffInfo,

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
    BuffFormula,

    /// Skill information.
    ///
    /// `time` contains `recharge`, `range0`, `range1` and `tooltiptime` as `[f32; 4]`.
    ///
    /// *Not used in realtime API.*
    SkillInfo,

    /// Skill action.
    ///
    /// `src_agent` contains the action.
    /// `dst_agent` contains at which millisecond.
    ///
    /// One per timing.
    ///
    /// *Not used in realtime API.*
    SkillTiming,

    /// Source agent breakbar state change.
    ///
    /// Value is [`u16`] game enum (active, recover, immune, none).
    ///
    /// *Not used in realtime API.*
    BreakbarState,

    /// Breakbar percentage.
    ///
    /// `value` contains percentage as float.
    ///
    /// *Not used in realtime API.*
    BreakbarPercent,

    /// Error.
    ///
    /// `time` contains the error message as an array of up to 32 characters.
    ///
    /// *Not used in realtime API.*
    Error,

    /// Source agent has tag.
    ///
    /// `value` is the id of the tag.
    /// Tag id is volatile, depends on game build.
    Tag,

    /// Source agent barrier change.
    ///
    /// `dst_agent` contains percentage as `percent * 10000`.
    /// For example 99.5% will be `9950`.
    ///
    /// *Not used in realtime API.*
    BarrierUpdate,

    /// Arc UI stats reset.
    ///
    /// `src_agent` contains the NPC id of the active log.
    ///
    /// *Not used in log EVTC.*
    StatReset,

    /// A custom event created by an extension (addon/plugin).
    Extension,

    /// Delayed combat event.
    ApiDelayed,

    /// Instance started.
    ///
    /// `src_agent` contains the time in ms at which the instance was likely started.
    InstanceStart,

    /// Tick rate.
    ///
    /// Every 500ms.
    /// `src_agent` is `25 - tickrate` (when `tickrate < 21`).
    Tickrate,

    /// Last 90% before down.
    ///
    /// `src_agent` is enemy agent that went down, `dst_agent` is time in ms since last 90%.
    /// For downs contribution.
    Last90BeforeDown,

    /// Effect created.
    ///
    /// `src_agent` is owner.
    /// `dst_agent` if located at agent.
    /// Otherwise `value` contains `[f32; 3]` XYZ, `affinity` contains `[f32; 2]` XY orientation, `pad61` contains [`f32`] Z orientation and `skill_id` contains the effect_id.
    /// If `is_flanking`, `duration` contains the tracking id.
    /// `is_shields` contains duration as [`u16`].
    /// If `effectid == 0`, end `is_shields` contains tracking id.
    ///
    /// *Not used in realtime API.*
    Effect,

    /// Id to GUID.
    ///
    /// `src_agent` contains [`u128`] (16 byte) persistent content guid, `overstack_value` is a variant of [`ContentLocal`](crate::ContentLocal), `skill_id` is content id.
    ///
    /// *Not used in realtime API.*
    IdToGUID,

    /// Log NPC changed.
    ///
    /// `value` contains the server Unix timestamp as `u32`.
    /// `buff_dmg` contains the local Unix timestamp.
    ///
    /// `src_agent` is species id.
    LogNPCUpdate,

    /// Used internally by ArcDPS.
    /// Should not appear anywhere.
    IdleEvent,

    /// A custom combat event created by an extension (addon/plugin).
    ///
    /// `skill_id` is treated as skill id and will be added to the EVTC skill table.
    ExtensionCombat,

    /// Unknown or invalid.
    #[default]
    Unknown,
}
