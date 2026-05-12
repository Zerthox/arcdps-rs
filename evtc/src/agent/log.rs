use crate::AgentKind;
use std::ffi::c_char;

/// An EVTC agent.
///
/// Could be a player, enemy, minion or other.
///
/// If `is_elite == 0xffffffff` and upper half of `prof == 0xffff`, the agent is a gadget with a pseudo id as lower half of `prof` (volatile id).
/// If `is_elite == 0xffffffff` and upper half of `prof != 0xffff`, the agent is an NPC with species id as lower half of `prof` (reliable id).
/// If `is_elite != 0xffffffff`, the agent is a player with Profession as `prof` and Elite Specialization as `is_elite`.
///
/// Gadgets do not have true ids and are generated through a combination of gadget parameters.
/// They will collide with NPCs and should be treated separately.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Agent {
    /// Unique agent id as assigned by ArcDPS.
    pub id: u64,

    /// Profession for player agents
    pub profession: u32,

    /// Elite specialization for player agents.
    pub is_elite: u32,

    /// Normalized Toughness attribute of the agent.
    pub toughness: i16,

    /// Normalized Concentration attribute of the agent.
    pub concentration: i16,

    /// Normalized Healing attribute of the agent.
    pub healing: i16,

    /// Hitbox width of the agent.
    pub hitbox_width: u16,

    /// Normalized Condition Damage attribute of the agent.
    pub condition: i16,

    /// Hitbox height of the agent.
    pub hitbox_height: u16,

    /// Name information for the agent.
    ///
    /// For players this is a combo string containing the character name, account name and subgroup.
    pub name: [c_char; 64],
}

impl Agent {
    /// Size of the name combo string.
    pub const NAME_SIZE: usize = 64;

    /// Determines the kind of agent.
    #[inline]
    pub const fn kind(&self) -> AgentKind {
        AgentKind::new(self.profession, self.is_elite)
    }
}
