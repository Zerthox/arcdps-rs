use super::AnimationKind;
use crate::{AgentId, Event, StateChange, TryExtract, extract::Extract};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Animation stop.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimationStop {
    /// Time of registering the activation.
    pub time: u64,

    /// Agent stopping the animation.
    pub agent: AgentId,

    /// Id of skill.
    pub skill_id: u32,

    /// Duration scaled with speed.
    pub duration_scaled: i32,

    /// Duration without speed scaling.
    pub duration_unscaled: i32,

    /// Animation progress.
    pub progress: AnimationProgress,
}

impl AnimationStop {
    #[inline]
    pub const fn kind(&self) -> AnimationKind {
        AnimationKind::new(self.skill_id)
    }
}

impl Extract for AnimationStop {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            skill_id: event.skill_id,
            duration_scaled: event.value,
            duration_unscaled: event.buff_dmg,
            progress: event.get_animation_progress(),
        }
    }
}

impl TryExtract for AnimationStop {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::AnimationStop
    }
}

/// Animation progress.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum AnimationProgress {
    /// None.
    None = 0,

    /// Stopped animation with reaching minimum of first trigger point or tooltip time.
    Minimum = 3,

    /// Stopped animation without reaching minimum of first trigger point or tooltip time.
    Cancel = 4,

    /// Animation completed fully.
    Reset = 5,

    /// Same as [`Minimum`](Self::Minimum) but on 0/uncertain expected duration.
    NoData = 6,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Animation stop trigger (debug only, subject to change).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum AnimationStopTrigger {
    None = 0,
    Instant = 1,
    MultiUnused = 2,
    Transition = 3,
    PartialUnused = 4,
    Ended = 5,
    Cancel = 6,
    StowDraw = 7,
    Interrupt = 8,
    Death = 9,
    Downed = 10,
    CrowdControl = 11,
    Command = 12,
    MotionSkill = 13,
    MoveDodge = 14,
    MotionSkillViaReset = 15,
    MoveSkill = 16,
    Stow = 17,
    AnyUused = 18,
    GadgetViaReset = 19,
    ManualExpiry = 20,
    Despawn = 21,
    ReturnControl = 22,
    Ready = 23,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}
