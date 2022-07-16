use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Whether the agent is an ally or enemy.
///
/// *Arc calls this "iff" for if friend/foe.*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum Affinity {
    /// Allied agent.
    Friend,

    /// Enemy agent.
    Foe,

    /// Uncertain whether ally or enemy.
    #[num_enum(default)]
    Unknown,
}

/// Strike types.
///
/// *Arc calls this "combat result".*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum Strike {
    /// Normal damage strike.
    ///
    /// No crit, no glance.
    Normal,

    /// Strike was critical.
    Crit,

    /// Strike was glancing.
    Glance,

    /// Strike was blocked.
    ///
    /// Due to Aegis, Chrono Shield 4 etc.
    Block,

    /// Strike was evaded.
    ///
    /// Due to dodge, Mesmer Sword 2 etc.
    Evade,

    /// Strike interrupted something.
    Interrupt,

    /// Strike was absorbed.
    ///
    /// Usually due to an invulnerability like Guardian Renewed Focus.
    Absorb,

    /// Strike missed.
    ///
    /// Due to blind etc.
    Blind,

    /// Skill killed the target.
    ///
    /// Not a damage strike.
    KillingBlow,

    /// Skill downed the target.
    ///
    /// Not a damage strike.
    Downed,

    /// Skill dealt breakbar damage.
    ///
    /// Not a damage strike.
    Breakbar,

    /// On-activation event.
    ///
    /// Not a damage strike.
    ///
    /// *Arc: Source hit target if damaging buff.*
    Activation,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// Skill activation (cast).
///
/// *Arc calls this "combat activation".*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Activation {
    /// Not used, different kind of event.
    None,

    /// Started skill/animation activation.
    Start,

    /// Unused as of 5th November 2019.
    QuicknessUnused,

    /// Stopped skill activation with reaching tooltip time.
    CancelFire,

    /// Stopped skill activation without reaching tooltip time.
    CancelCancel,

    /// Animation completed fully.
    Reset,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// Combat state change.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
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
    /// `0`/`1` for underwater weapons and `4`/`5` for land weapons.
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
    /// Value contains the current targetable state.
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
    /// `dst_agent` until `buff_dmg` is 16 byte (`u8`) guid.
    ///
    /// Given in client form, needs minor rearrange for API form.
    Guild,

    /// Buff information.
    ///
    /// If `is_flanking` probably invulnerable.
    /// If `is_shields` probably invert.
    ///
    /// Offcycle contains the category.
    /// `pad61` contains the stacking type.
    /// `pad62` contains the probably resistance.
    /// `src_master_instid` contains the max stacks.
    /// `overstack_value` contains the duration cap.
    ///
    /// *Not used in realtime API.*
    BuffInfo,

    /// Buff formula.
    ///
    /// Time contains `type`, `attr1`, `attr2`, `param1`, `param2`, `param3`, `trait_src` and `trait_self` as array of 8 floats.
    /// Source instance id contains `buff_src` and `buff_self` as array of 2 floats.
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
    /// Time contains `recharge`, `range0`, `range1` and `tooltiptime` as array of 4 floats.
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

    /// Combat event with state change byte set to this.
    Extension,

    /// Combat event with state change byte set to this.
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
    /// `dst_agent` if at agent, else `&value = float[3] xyz`, `&iff = float[2] xy` orient, `&pad61 = float[1] z` orient, `skillid = effectid`.
    /// If `is_flanking`: `duration = trackingid`.
    /// `&is_shields = uint16` duration.
    /// If `effectid == 0`, end `&is_shields = trackingid`.
    ///
    /// *Not used in realtime API.*
    Effect,

    /// Id to GUID.
    ///
    /// `&src_agent = 16byte` persistent content guid, `overstack_value` is of contentlocal enum, `skillid` is content id.
    ///
    /// *Not used in realtime API.*
    IdToGUID,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// Combat buff remove.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffRemove {
    /// Not used, different kind of event.
    None,

    /// Last or all stacks removed.
    ///
    /// Sent by server.
    All,

    /// Single stack removed.
    ///
    /// Happens for each stack on cleanse.
    ///
    /// Sent by server.
    Single,

    /// Single stack removed.
    ///
    /// Automatically by Arc on out of combat or all stack.
    /// Ignore for strip/cleanse calculation.
    /// Use for in/out volume.
    Manual,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// Combat buff cycle.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum BuffCycle {
    /// Damage happened on tick timer.
    Cycle,

    /// Damage happened outside tick timer (resistable).
    NotCycle,

    /// Retired since May 2021.
    NotCycleOrResist,

    /// Damage happened to target on hitting target.
    NotCycleDmgToTargetOnHit,

    /// Damage happened to source on hitting target.
    NotCycleDmgToSourceOnHit,

    /// Damage happened to target on source losing a stack.
    NotCycleDmgToTargetOnStackRemove,

    #[num_enum(default)]
    Unknown,
}

/// ArcDPS custom skill ids.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u16)]
pub enum CustomSkill {
    /// Resurrect skill.
    ///
    /// Not custom but important and unnamed.
    Resurrect = 1066,

    /// Bandage downstate skill.
    ///
    /// Personal healing only.
    Bandage = 1175,

    /// Dodge skill.
    ///
    /// Will occur in `is_activation == normal` event.
    Dodge = 65001,
}

/// Buff info category.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum BuffCategory {
    Boon = 0,
    Any = 1,
    Condition = 2,
    Food = 4,
    Upgrade = 6,
    Boost = 8,
    Trait = 11,
    Enhancement = 13,
    Stance = 16,
}

/// Buff formula attributes.
// TODO: document unclear attributes
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u16)]
pub enum Attribute {
    None,

    Power,
    Precision,
    Toughness,
    Vitality,
    Ferocity,
    Healing,
    Condition,
    Concentration,
    Expertise,

    Armor,
    Agony,
    StatInc,
    FlatInc,
    PhysInc,
    CondInc,
    PhysRec,
    CondRec,
    Attackspeed,
    SiphonInc,
    SiphonRec,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown = 65535,
}
