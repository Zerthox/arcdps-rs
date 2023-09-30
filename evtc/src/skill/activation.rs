use crate::{extract::Extract, AgentId, Event, EventCategory, Position, TryExtract};
use num_enum::{FromPrimitive, IntoPrimitive};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Activation (skill cast) event.
///
/// Only animated skill casts are captured by ArcDPS.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ActivationEvent {
    /// Time of registering the activation.
    pub time: u64,

    /// Agent casting the skill.
    pub agent: AgentId,

    /// Id of casted skill.
    pub skill_id: u32,

    /// Kind of activation state change.
    pub activation: Activation,

    /// Activation duration.
    pub duration: i32,

    /// Full activation duration.
    pub full_duration: i32,

    /// Target location, if applicable.
    pub target: Position,
}

impl Extract for ActivationEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let [x, y]: [f32; 2] = transmute(event.dst_agent);
        let z = f32::from_bits(event.overstack_value);
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            skill_id: event.skill_id,
            activation: event.get_activation(),
            duration: event.value,
            full_duration: event.buff_dmg,
            target: Position::new(x, y, z),
        }
    }
}

impl TryExtract for ActivationEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.categorize() == EventCategory::Activation
    }
}

/// Skill activation (cast).
///
/// *Arc calls this "combat activation".*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum Activation {
    /// Not used, different kind of event.
    None = 0,

    /// Started skill/animation activation.
    Start = 1,

    /// Unused as of 5th November 2019.
    QuicknessUnused = 2,

    /// Stopped skill activation with reaching tooltip time.
    CancelFire = 3,

    /// Stopped skill activation without reaching tooltip time.
    CancelCancel = 4,

    /// Animation completed fully.
    Reset = 5,

    /// Unknown or invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}
