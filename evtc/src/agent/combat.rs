use crate::{
    extract::Extract, AgentId, Event, Profession, Specialization, StateChange, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Agent entered combat.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnterCombatEvent {
    /// Time of registering the combat enter.
    pub time: u64,

    /// Agent that entered combat.
    pub agent: AgentId,

    /// Agent subgroup.
    pub subgroup: u64,

    /// Agent profession.
    pub profession: Profession,

    /// Agent elite specialization.
    pub elite: Specialization,
}

impl Extract for EnterCombatEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            subgroup: event.dst_agent,
            profession: event.value.cast_unsigned().into(),
            elite: event.buff_dmg.cast_unsigned().into(),
        }
    }
}

impl TryExtract for EnterCombatEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::EnterCombat
    }
}
