use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

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
    /// `prof` is the Profession and `elite` the Elite-Specialization.
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
            let (lower, upper): (u16, u16) = unsafe { transmute(prof) };
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
