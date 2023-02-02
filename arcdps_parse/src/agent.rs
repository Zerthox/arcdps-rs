use crate::{
    util::{read_string_buffer, Endian},
    Parse,
};
use byteorder::ReadBytesExt;
use std::io;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Agent {
    /// Name of the agent.
    ///
    /// This is a combo string for players: `character name \0 account name \0 subgroup \0`.
    pub name: String,

    /// Address (id) of the agent.
    pub address: u64,

    /// Profession for player agents
    pub profession: u32,

    /// Elite specialization for player agents.
    pub is_elite: u32,

    /// Hitbox width of the agent.
    pub hitbox_width: u16,

    /// Hitbox height of the agent.
    pub hitbox_height: u16,

    /// Normalized Toughness attribute of the agent.
    pub toughness: u16,

    /// Normalized Concentration attribute of the agent.
    pub concentration: u16,

    /// Normalized Healing attribute of the agent.
    pub healing: u16,

    /// Normalized Condition Damage attribute of the agent.
    pub condition: u16,
}

impl Parse for Agent {
    type Error = crate::ParseError;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        let address = input.read_u64::<Endian>()?;
        let profession = input.read_u32::<Endian>()?;
        let is_elite = input.read_u32::<Endian>()?;
        let toughness = input.read_u16::<Endian>()?;
        let concentration = input.read_u16::<Endian>()?;
        let healing = input.read_u16::<Endian>()?;
        let hitbox_width = input.read_u16::<Endian>()?;
        let condition = input.read_u16::<Endian>()?;
        let hitbox_height = input.read_u16::<Endian>()?;
        let name = read_string_buffer::<64>(input)?;

        // padding added by c
        input.read_u32::<Endian>()?;

        Ok(Self {
            name,
            address,
            profession,
            is_elite,
            hitbox_width,
            hitbox_height,
            toughness,
            concentration,
            healing,
            condition,
        })
    }
}
