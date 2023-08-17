use crate::CombatEvent;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::mem;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Ids for an agent.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentId {
    /// Agent id as assigned by Arc.
    pub agent: u64,

    /// Instance id of the agent as appears in game at time of event.
    pub instance_id: u16,

    /// If agent has a master (e.g. is minion), will be equal to instance id of master, zero otherwise.
    pub master_instance_id: u16,
}

impl AgentId {
    /// Creates new agent id information.
    #[inline]
    pub const fn new(agent: u64, instance_id: u16, master_instance_id: u16) -> Self {
        Self {
            agent,
            instance_id,
            master_instance_id,
        }
    }

    /// Creates new agent id information without a master.
    #[inline]
    pub const fn without_master(agent: u64, instance_id: u16) -> Self {
        Self::new(agent, instance_id, 0)
    }

    /// Creates new agent id information from the [`CombatEvent`] source agent.
    #[inline]
    pub const fn from_src(event: &CombatEvent) -> Self {
        Self::new(
            event.src_agent,
            event.src_instance_id,
            event.src_master_instance_id,
        )
    }

    /// Creates new agent id information from the [`CombatEvent`] destination agent.
    #[inline]
    pub const fn from_dst(event: &CombatEvent) -> Self {
        Self::new(
            event.dst_agent,
            event.dst_instance_id,
            event.dst_master_instance_id,
        )
    }
}

/// Possible agent kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
pub enum AgentKind {
    /// Agent is a player.
    ///
    /// `prof` contains the Profession and `elite` the Elite-Specialization.
    Player,

    /// Agent is an NPC.
    ///
    /// The included id is the (reliable) species id.
    Npc(u16),

    /// Agent is a gadget.
    ///
    /// The included id is a volatile pseudo id.
    Gadget(u16),
}

impl AgentKind {
    /// Determines the kind of agent for the given profession and elite.
    #[inline]
    pub const fn new(prof: u32, elite: u32) -> Self {
        if elite == u32::MAX {
            let (lower, upper): (u16, u16) = unsafe { mem::transmute(prof) };
            if upper == u16::MAX {
                AgentKind::Gadget(lower)
            } else {
                AgentKind::Npc(lower)
            }
        } else {
            AgentKind::Player
        }
    }
}

impl From<(u32, u32)> for AgentKind {
    #[inline]
    fn from((prof, elite): (u32, u32)) -> Self {
        Self::new(prof, elite)
    }
}

/// Whether the agent is an ally or enemy.
///
/// *Arc calls this "iff" for if friend/foe.*
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum Affinity {
    /// Allied agent.
    Friend = 0,

    /// Enemy agent.
    Foe = 1,

    /// Invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}
