use crate::{
    extract::{transmute_field, Extract},
    Event, Position, StateChange, TryExtract,
};
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Squad (ground) marker placed or removed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SquadMarkerEvent {
    /// Time of registering the event.
    pub time: u64,

    /// The squad marker that was modified.
    pub marker: SquadMarker,

    /// The position of the squad marker.
    pub position: Position,
}

impl SquadMarkerEvent {
    /// Whether the marker was removed.
    #[inline]
    pub fn is_remove(&self) -> bool {
        self.position == Position::new(f32::INFINITY, f32::INFINITY, f32::INFINITY)
    }
}

impl Extract for SquadMarkerEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let pos = transmute_field!(event.src_agent as [f32; 3]);
        Self {
            time: event.time,
            marker: event.skill_id.into(),
            position: pos.into(),
        }
    }
}

impl TryExtract for SquadMarkerEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::SquadMarker
    }
}

/// Squad marker.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u32)]
pub enum SquadMarker {
    Arrow = 0,

    Circle = 1,

    Heart = 2,

    Square = 3,

    Star = 4,

    Swirl = 5,

    Triangle = 6,

    X = 7,

    #[num_enum(catch_all)]
    Unknown(u32),
}
