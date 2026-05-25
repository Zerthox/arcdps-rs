use super::AnimationKind;
use crate::{AgentId, Event, StateChange, TryExtract, extract::Extract};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Animation start.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimationStart {
    /// Time of registering the activation.
    pub time: u64,

    /// Agent starting the animation.
    pub agent: AgentId,

    /// Target agent, if applicable.
    pub target: AgentId,

    /// Id of skill.
    pub skill_id: u32,

    /// Reference id.
    ///
    /// Emote id for emote.
    /// Item id for bundle pickup.
    pub reference_id: u32,

    /// Duration until minimum of last significant trigger.
    pub duration_execute: i32,

    /// Duration until control is returned.
    pub duration_control: i32,
}

impl AnimationStart {
    /// Returns the [`AnimationKind`].
    #[inline]
    pub const fn kind(&self) -> AnimationKind {
        AnimationKind::new(self.skill_id)
    }
}

impl Extract for AnimationStart {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            target: AgentId::from_dst(event),
            skill_id: event.skill_id,
            reference_id: event.overstack_value,
            duration_execute: event.value,
            duration_control: event.buff_dmg,
        }
    }
}

impl TryExtract for AnimationStart {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::AnimationStart
    }
}

/// Animation start trigger (debug only, subject to change).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum AnimationStartTrigger {
    None = 0,
    Skill = 1,
    Dodge = 2,
    StowDraw = 3,
    MoveSkill = 4,
    MotionSkill = 5,
    GadgetInteract = 6,
    Emote = 7,
    Pickup = 8,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}
