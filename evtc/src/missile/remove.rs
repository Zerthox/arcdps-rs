use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Missile created.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MissileRemove {
    /// Time of registering the missile.
    pub time: u64,

    /// Related agent.
    pub agent: AgentId,

    /// Total friendly fire damage.
    pub friendly_fire: i32,

    /// Associated skill id.
    pub skill_id: u32,

    /// Whether at least one enemy was hit along the way.
    pub hit_enemy: u8,

    /// Trackable id to identify missile in other events.
    pub tracking_id: u32,
}

impl Extract for MissileRemove {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let friendly_fire = event.value;
        let skill_id = event.skill_id;
        let hit_enemy = event.is_flanking;

        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            friendly_fire,
            skill_id,
            hit_enemy,
            tracking_id: event.get_pad_id(),
        }
    }
}

impl TryExtract for MissileRemove {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::MissileCreate
    }
}
