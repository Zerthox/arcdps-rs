use crate::{Affinity, AgentId, CombatEvent};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Information common to combat events.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommonEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that caused the event.
    pub src: AgentId,

    /// Agent the event happened to.
    pub dst: AgentId,

    /// Skill id of the relevant skill (can be zero).
    pub skill_id: u32,

    /// Current affinity of `src` and `dst`.
    ///
    /// *Arc calls this "iff" for if friend/foe.*
    pub affinity: Affinity,

    /// Whether `src` is above 90% Health.
    pub is_ninety: u8,

    /// Whether `dst` is below 50% Health.
    pub is_fifty: u8,

    /// Whether `src` is moving at time of event.
    pub is_moving: u8,

    /// Whether `src` is flanking at time of event.
    ///
    /// The value lies in a range of `1` to `135` degrees where `135` is rear.
    pub is_flanking: u8,
}

impl From<&CombatEvent> for CommonEvent {
    #[inline]
    fn from(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            src: AgentId::from_src(event),
            dst: AgentId::from_dst(event),
            skill_id: event.skill_id,
            affinity: event.affinity,
            is_ninety: event.is_ninety,
            is_fifty: event.is_fifty,
            is_moving: event.is_moving,
            is_flanking: event.is_flanking,
        }
    }
}
