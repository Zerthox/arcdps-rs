//! Bindings & utilities for agent weapon sets.

use crate::{extract::Extract, AgentId, CombatEvent, StateChange, TryExtract};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Active weapon set changed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeaponSwapEvent {
    pub agent: AgentId,
    pub weapon_set: WeaponSet,
}

impl Extract for WeaponSwapEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            agent: AgentId::from_src(event),
            weapon_set: event.dst_agent.into(),
        }
    }
}

impl TryExtract for WeaponSwapEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::WeaponSwap
    }
}

/// Agent weapon set.
///
/// Typically used with a [`CombatEvent`] with [`StateChange::WeaponSwap`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
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

    /// Unknown.
    #[num_enum(catch_all)]
    Unknown(u64),
}
