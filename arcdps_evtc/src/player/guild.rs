use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Agent is in guild.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GuildEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that is in guild.
    pub agent: AgentId,

    /// Guild id in client form.
    ///
    /// Needs minor rearrange for GW2 API form.
    pub guild: u128,
}

impl Extract for GuildEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            guild: transmute((event.dst_agent, event.value, event.buff_dmg)),
        }
    }
}

impl TryExtract for GuildEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Guild
    }
}
