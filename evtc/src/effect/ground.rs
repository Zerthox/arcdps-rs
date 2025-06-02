use crate::extract::transmute_field;
use crate::{extract::Extract, Event, StateChange, TryExtract};
use crate::{AgentId, Position};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Effect information from an [`Event`] with [`StateChange::EffectGroundCreate`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroundEffect {
    /// Time of registering the effect.
    pub time: u64,

    /// Owner of the effect.
    pub owner: AgentId,

    /// Id of the effect.
    ///
    /// Use to map to a GUID using [`StateChange::IdToGUID`] events.
    pub effect_id: u32,

    /// Location of the effect.
    pub location: Position,

    /// Effect orientation.
    pub orientation: Position,

    /// Duration of the effect in milliseconds.
    pub duration: u32,

    /// Effect flags.
    pub flags: u8,

    /// Whether the effect is on a moving platform.
    pub moving_platform: u8,

    /// Effect scale.
    pub scale: f32,

    /// Trackable id for effect remove.
    pub tracking_id: u32,
}

impl Extract for GroundEffect {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let [pos_x, pos_y, pos_z, orient_x, orient_y, orient_z] =
            transmute_field!(event.dst_agent as [i16; 6]);
        let effect_id = event.skill_id;
        let duration = transmute_field!(event.affinity as u32);
        let flags = event.is_buffremove;
        let moving_platform = event.is_flanking;
        let scale = transmute_field!(event.is_shields as i16);

        Self {
            time: event.time,
            owner: AgentId::from_src(event),
            effect_id,
            location: Position::from_scaled_i16s(pos_x, pos_y, pos_z, 10.0),
            orientation: Position::from_scaled_i16s(orient_x, orient_y, orient_z, 1.0 / 1000.0),
            duration,
            flags,
            moving_platform,
            scale: scale as f32 / 1000.0,
            tracking_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for GroundEffect {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::EffectGroundCreate
    }
}

/// Effect information from an [`Event`] with [`StateChange::EffectGroundRemove`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroundEffectRemove {
    /// Time of registering the effect.
    pub time: u64,

    /// Trackable id for effect remove.
    pub tracking_id: u32,
}

impl Extract for GroundEffectRemove {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            tracking_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for GroundEffectRemove {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::EffectGroundRemove
    }
}
