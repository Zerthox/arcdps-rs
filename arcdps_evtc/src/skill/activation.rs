use crate::{CombatEvent, Extract, Position};
use num_enum::{FromPrimitive, IntoPrimitive};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// A direct damage event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ActivationEvent {
    pub time: u64,
    pub src_agent: u64,
    pub src_instance_id: u16,
    pub src_master_instance_id: u16,
    pub skill_id: u32,
    pub kind: Activation,
    pub duration: i32,
    pub full_duration: i32,
    pub target: Position,
}

impl Extract for ActivationEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        let [x, y]: [f32; 2] = transmute(event.dst_agent);
        let z = f32::from_bits(event.overstack_value);
        Self {
            time: event.time,
            src_agent: event.src_agent,
            src_instance_id: event.src_instance_id,
            src_master_instance_id: event.src_master_instance_id,
            skill_id: event.skill_id,
            kind: event.is_activation,
            duration: event.value,
            full_duration: event.buff_dmg,
            target: Position::new(x, y, z),
        }
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
