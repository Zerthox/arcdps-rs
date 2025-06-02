use crate::extract::transmute_field;
use crate::AgentId;
use crate::{extract::Extract, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Effect information from an [`Event`] with [`StateChange::EffectAgentCreate`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentEffect {
    /// Time of registering the effect.
    pub time: u64,

    /// Agent related to the effect.
    pub agent: AgentId,

    /// Id of the effect.
    ///
    /// Use to map to a GUID using [`StateChange::IdToGUID`] events.
    pub effect_id: u32,

    /// Duration of the effect in milliseconds.
    pub duration: u32,

    /// Trackable id for effect remove.
    pub tracking_id: u32,
}

impl Extract for AgentEffect {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let effect_id = event.skill_id;
        let duration = transmute_field!(event.affinity as u32);

        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            effect_id,
            duration,
            tracking_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for AgentEffect {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::EffectAgentCreate
    }
}

/// Effect information from an [`Event`] with [`StateChange::EffectAgentRemove`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentEffectRemove {
    /// Time of registering the effect.
    pub time: u64,

    /// Agent related to the effect.
    pub agent: AgentId,

    /// Trackable id for effect remove.
    pub tracking_id: u32,
}

impl Extract for AgentEffectRemove {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            tracking_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for AgentEffectRemove {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::EffectAgentRemove
    }
}
