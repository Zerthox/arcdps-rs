use crate::{CombatEvent, StateChange};
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Skill activation (cast).
///
/// *Arc calls this "combat activation".*
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
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
    #[default]
    Unknown,
}

/// ArcDPS custom skill ids.
pub enum CustomSkill {}

impl CustomSkill {
    /// Resurrect skill.
    ///
    /// Not custom but important and unnamed.
    pub const RESURRECT: u32 = 1066;

    /// Bandage downstate skill.
    ///v
    /// Personal healing only.
    pub const BANDAGE: u32 = 1175;

    /// Dodge skill.
    ///
    /// Will occur in `is_activation == normal` event.
    pub const DODGE: u32 = 65001;
}

/// Agent weapon set.
///
/// Typically used with a [`CombatEvent`] with [`StateChange::WeaponSwap`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u64)]
pub enum WeaponSet {
    /// First underwater weapon set.
    Water1 = 0,

    /// Second underwater weapon set.
    Water2 = 1,

    /// Bundle or kit weapon set.
    Bundle = 2,

    /// Transform weapon set.
    Transform = 3,

    /// First land weapon set.
    Land1 = 4,

    /// Second land weapon set.
    Land2 = 5,
}

/// Skill information from a [`CombatEvent`] with [`StateChange::SkillInfo`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SkillInfo {
    pub recharge: f32,
    pub range0: f32,
    pub range1: f32,
    pub tooltip_time: f32,
}

impl TryFrom<&CombatEvent> for SkillInfo {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::SkillInfo => {
                let [recharge, range0, range1, tooltip_time]: [f32; 4] =
                    unsafe { transmute((event.time, event.src_agent)) };
                Ok(Self {
                    recharge,
                    range0,
                    range1,
                    tooltip_time,
                })
            }

            _ => Err(()),
        }
    }
}

/// Skill timing from a [`CombatEvent`] with [`StateChange::SkillTiming`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SkillTiming {
    pub action: u64,
    pub millisecond: u64,
}

impl TryFrom<&CombatEvent> for SkillTiming {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::SkillTiming => Ok(Self {
                action: event.src_agent,
                millisecond: event.dst_agent,
            }),

            _ => Err(()),
        }
    }
}
