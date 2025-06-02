use crate::{
    extract::{transmute_field, Extract},
    AgentId, Event, Position, StateChange, TryExtract,
};
use bitflags::bitflags;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Missile created.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MissileLaunch {
    /// Time of registering the missile.
    pub time: u64,

    /// Agent creating the missile.
    pub source: AgentId,

    /// Target of the missile, if set and in range.
    pub target: AgentId,

    /// Target location.
    pub target_location: Position,

    /// Current location.
    pub current_location: Position,

    /// Associated skill id.
    pub skill_id: u32,

    /// Missile motion type.
    pub motion: u8,

    /// Range or radius depending on the missile's motion.
    pub range: i16,

    /// Missile flags on launch.
    pub flags: MissileFlags,

    /// Missile speed.
    pub speed: i16,

    /// Trackable id to identify missile in other events.
    pub tracking_id: u32,
}

impl Extract for MissileLaunch {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let [target_x, target_y, target_z, cur_x, cur_y, cur_z] =
            transmute_field!(event.value as [i16; 6]);
        let skill_id = event.skill_id;

        let motion = event.affinity;
        let range = transmute_field!(event.result as i16);
        let flags = transmute_field!(event.is_buffremove as u32);
        let speed = transmute_field!(event.is_shields as i16);

        Self {
            time: event.time,
            source: AgentId::from_src(event),
            target: AgentId::from_dst(event),
            target_location: Position::from_scaled_i16s(target_x, target_y, target_z, 10.0),
            current_location: Position::from_scaled_i16s(cur_x, cur_y, cur_z, 10.0),
            skill_id,
            motion,
            range,
            flags: MissileFlags::from_bits_retain(flags),
            speed,
            tracking_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for MissileLaunch {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::MissileCreate
    }
}

bitflags! {
    /// Missile flags on launch.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct MissileFlags : u32 {
        const _ = !0;
    }
}
