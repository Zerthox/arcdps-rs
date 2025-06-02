use crate::{
    extract::{transmute_field, Extract},
    AgentId, Event, Position, StateChange, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Missile created.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MissileCreate {
    /// Time of registering the missile.
    pub time: u64,

    /// Agent creating the missile.
    pub source: AgentId,

    /// Missile location.
    pub location: Position,

    /// Skin id for players.
    pub skin_id: u32,

    /// Associated skill id.
    pub skill_id: u32,

    /// Trackable id to identify missile in other events.
    pub tracking_id: u32,
}

impl Extract for MissileCreate {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let [x, y, z] = transmute_field!(event.value as [i16; 3]);
        let skin_id = event.overstack_value;
        let skill_id = event.skill_id;

        Self {
            time: event.time,
            source: AgentId::from_src(event),
            location: Position::from_scaled_i16s(x, y, z, 10.0),
            skill_id,
            skin_id,
            tracking_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for MissileCreate {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::MissileCreate
    }
}
