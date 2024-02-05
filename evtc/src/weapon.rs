//! Bindings & utilities for agent weapon sets.

use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Active weapon set changed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeaponSwapEvent {
    /// Time of registering the weapon swap.
    pub time: u64,

    /// Agent that swapped weapon sets.
    pub agent: AgentId,

    /// New weapon set.
    pub weapon_set: WeaponSet,
}

impl Extract for WeaponSwapEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            weapon_set: event.dst_agent.into(),
        }
    }
}

impl TryExtract for WeaponSwapEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::WeaponSwap
    }
}

/// Agent weapon set.
///
/// Typically used with an [`Event`] with [`StateChange::WeaponSwap`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
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
