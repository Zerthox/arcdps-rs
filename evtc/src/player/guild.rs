use crate::{
    extract::{transmute_field, Extract},
    guid::GUID,
    AgentId, Event, StateChange, TryExtract,
};

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

    /// Guild id in GW2 API form.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_guid"))]
    pub guild: GUID,
}

impl Extract for GuildEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            guild: transmute_field!(event.dst_agent as GUID),
        }
    }
}

impl TryExtract for GuildEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Guild
    }
}
