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
/// Could be a player, NPC, minion or other.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Agent {
    pub name: String,
    pub address: u64,
    pub profession: u32,
    pub is_elite: u32,
    pub hitbox_width: u16,
    pub hitbox_height: u16,
    pub toughness: u16,
    pub concentration: u16,
    pub healing: u16,
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
