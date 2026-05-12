//! Skill bindings & utilities.

mod info;
mod timing;

pub use self::info::*;
pub use self::timing::*;

/// ArcDPS custom skill ids.
pub enum CustomSkill {}

impl CustomSkill {
    /// Dodge skill.
    pub const DODGE: u32 = 23275;

    /// Defiance damage.
    pub const DEFIANCE_DAMAGE: u32 = 23276;

    /// Self cast.
    pub const SELF_CAST1: u32 = 23277;

    /// Enemy cast.
    pub const ENEMY_CAST1: u32 = 23278;

    /// Self cast.
    pub const SELF_CAST2: u32 = 23279;

    /// Enemy cast.
    pub const ENEMY_CAST2: u32 = 23280;

    /// Self cast.
    pub const SELF_CAST3: u32 = 23281;

    /// Enemy cast.
    pub const ENEMY_CAST3: u32 = 23282;

    /// Breakbar (unused).
    pub const BREAKBAR_UNUSED: u32 = 23283;

    /// Weapon draw.
    pub const WEAPON_DRAW: u32 = 23284;

    /// Weapon stow.
    pub const WEAPON_STOW: u32 = 23285;

    /// Generic block.
    pub const GENERIC_BLOCK: u32 = 23286;

    /// Generic damage.
    pub const GENERIC_DAMAGE: u32 = 23287;

    /// Generic kill.
    pub const GENERIC_KILL: u32 = 23288;

    /// Generic down.
    pub const GENERIC_DOWN: u32 = 23289;

    /// Generic evade.
    pub const GENERIC_EVADE: u32 = 23290;

    /// Generic interrupt.
    pub const GENERIC_INTERRUPT: u32 = 23291;

    /// Generic absorb.
    pub const GENERIC_ABSORB: u32 = 23292;

    /// Generic miss.
    pub const GENERIC_MISS: u32 = 23293;

    /// Generic knockdown.
    pub const GENERIC_KNOCKDOWN: u32 = 23294;

    /// Generic knockback or oull.
    pub const GENERIC_KNOCKBACK_PULL: u32 = 23295;

    /// Generic float on land.
    pub const GENERIC_FLOAT_LAND: u32 = 23296;

    /// Generic launch.
    pub const GENERIC_LAUNCH: u32 = 23297;

    /// Generic float or sink in water (unused).
    pub const GENERIC_WATER_FLOAT_SINK_UNUSED: u32 = 23298;

    /// Generic CC buff.
    pub const GENERIC_CC_BUFF: u32 = 23299;

    /// Generic stagger.
    pub const GENERIC_STAGGER: u32 = 23300;

    /// Generic invalid.
    pub const GENERIC_INVALID: u32 = 23301;

    /// Gadget interact.
    pub const GADGET_INTERACT: u32 = 23302;

    /// Emote.
    pub const EMOTE: u32 = 23303;

    /// Generic float in water.
    pub const GENERIC_FLOAT_WATER: u32 = 23304;

    /// Generic sink.
    pub const GENERIC_SINK: u32 = 23305;

    /// Generic lockout.
    pub const GENERIC_LOCKOUT: u32 = 23306;

    /// Generic fear.
    pub const GENERIC_FEAR: u32 = 23307;
}
