use crate::{AgentId, Event, StateChange, TryExtract, extract::Extract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Transformation change.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransformationEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that caused the event.
    pub source: AgentId,

    /// Agent the event happened to.
    pub target: AgentId,

    /// Id of the transformation.
    ///
    /// Use to map to a GUID using [`StateChange::IdToGUID`] events.
    pub transformation_id: u32,
}

impl Extract for TransformationEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            source: AgentId::from_src(event),
            target: AgentId::from_dst(event),
            transformation_id: event.skill_id,
        }
    }
}

impl TryExtract for TransformationEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Transformation
    }
}
