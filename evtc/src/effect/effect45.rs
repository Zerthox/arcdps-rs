use crate::{
    extract::{transmute_field, Extract},
    AgentId, Event, Position, StateChange, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use super::effect51::EffectLocation;

/// Effect information from an [`Event`] with [`StateChange::Effect45`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Effect45 {
    /// Time of registering the effect.
    pub time: u64,

    /// Id of the effect.
    ///
    /// Use to map to a GUID using [`StateChange::IdToGUID`] events.
    pub effect_id: u32,

    /// Owner of the effect.
    pub owner: AgentId,

    /// Location of the effect.
    pub location: EffectLocation,

    /// Orientation of the effect as 3 dimensional vector.
    pub orientation: Position,

    /// Duration of the effect in time or as tracking id.
    pub duration: EffectDuration,
}

impl Effect45 {
    /// Checks whether this is the end of an effect.
    #[inline]
    pub fn is_end(&self) -> bool {
        self.effect_id == 0
    }
}

impl Extract for Effect45 {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let effect_id = event.skill_id;
        let [x, y] = transmute_field!(event.affinity as [f32; 2]);
        let z = transmute_field!(event.pad61 as f32);
        let duration = transmute_field!(event.is_shields as u16);

        Self {
            time: event.time,
            effect_id,
            owner: AgentId::from_src(event),
            location: EffectLocation::extract(event),
            orientation: [x, y, z].into(),
            duration: if event.is_flanking != 0 || effect_id == 0 {
                EffectDuration::TrackingId(duration)
            } else {
                EffectDuration::Time(duration)
            },
        }
    }
}

impl TryExtract for Effect45 {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Effect51
    }
}

/// Duration of an effect in time or as a tracking id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EffectDuration {
    /// Duration as time in milliseconds.
    Time(u16),

    /// Duration as tracking id.
    TrackingId(u16),
}
