use crate::event::Event;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Ids for an agent.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentId {
    /// Agent id as assigned by Arc.
    pub id: u64,

    /// Instance id of the agent as appears in game at time of event.
    pub instance_id: u16,

    /// If agent has a master (e.g. is minion), will be equal to instance id of master, zero otherwise.
    pub master_instance_id: u16,
}

impl AgentId {
    /// Creates new agent id information.
    #[inline]
    pub const fn new(id: u64, instance_id: u16, master_instance_id: u16) -> Self {
        Self {
            id,
            instance_id,
            master_instance_id,
        }
    }

    /// Creates new agent id information without a master.
    #[inline]
    pub const fn without_master(id: u64, instance_id: u16) -> Self {
        Self::new(id, instance_id, 0)
    }

    /// Creates new agent id information from the [`Event`] source agent.
    #[inline]
    pub const fn from_src(event: &Event) -> Self {
        Self::new(
            event.src_agent,
            event.src_instance_id,
            event.src_master_instance_id,
        )
    }

    /// Creates new agent id information from the [`Event`] destination agent.
    #[inline]
    pub const fn from_dst(event: &Event) -> Self {
        Self::new(
            event.dst_agent,
            event.dst_instance_id,
            event.dst_master_instance_id,
        )
    }

    /// Returns whether the agent has a master.
    #[inline]
    pub const fn has_master(&self) -> bool {
        self.master_instance_id != 0
    }
}
