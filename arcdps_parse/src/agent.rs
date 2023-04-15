use crate::{
    util::{read_string_buffer, write_string_buffer, Endian},
    Parse, ParseError, Save,
};
use arcdps_evtc::AgentKind;
use byteorder::{ReadBytesExt, WriteBytesExt};
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
    /// Address (id) of the agent.
    pub address: u64,

    /// Name information for the agent.
    ///
    /// For players this is a combo string containing the character name, account name and subgroup.
    pub name: Vec<String>,

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

impl Agent {
    /// Size of the name combo string.
    pub const NAME_SIZE: usize = 64;

    /// Determines the kind of agent.
    pub const fn kind(&self) -> AgentKind {
        AgentKind::new(self.profession, self.is_elite)
    }

    /// Parses name information from the input.
    fn parse_name(input: &mut impl io::Read) -> Result<Vec<String>, ParseError> {
        let string = read_string_buffer::<{ Self::NAME_SIZE }>(input)?;
        Ok(string
            .split('\0')
            .filter(|part| !part.is_empty())
            .map(|part| part.to_string())
            .collect())
    }

    /// Saves name information to the output.
    fn save_name(&self, output: &mut impl io::Write) -> Result<(), io::Error> {
        let string = self.name.join("\0");
        write_string_buffer::<{ Self::NAME_SIZE }>(output, &string)
    }
}

impl Parse for Agent {
    type Error = ParseError;

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

        let name = Self::parse_name(input)?;

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

impl Save for Agent {
    type Error = io::Error;

    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error> {
        output.write_u64::<Endian>(self.address)?;
        output.write_u32::<Endian>(self.profession)?;
        output.write_u32::<Endian>(self.is_elite)?;
        output.write_u16::<Endian>(self.toughness)?;
        output.write_u16::<Endian>(self.concentration)?;
        output.write_u16::<Endian>(self.healing)?;
        output.write_u16::<Endian>(self.hitbox_width)?;
        output.write_u16::<Endian>(self.condition)?;
        output.write_u16::<Endian>(self.hitbox_height)?;

        self.save_name(output)?;

        // padding added by c
        output.write_u32::<Endian>(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_name() {
        let name: Vec<String> = vec!["Character".into(), ":Account.1234".into(), "1".into()];
        let data: &[u8; Agent::NAME_SIZE] = b"Character\0:Account.1234\x001\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

        let parsed = Agent::parse_name(io::Cursor::new(data.as_slice()).get_mut())
            .expect("failed to parse agent name");
        assert_eq!(name, parsed, "incorrect agent name");

        let agent = Agent {
            address: 0,
            name,
            profession: 0,
            is_elite: 0,
            hitbox_width: 0,
            hitbox_height: 0,
            toughness: 0,
            concentration: 0,
            healing: 0,
            condition: 0,
        };

        let mut buffer = [123u8; Agent::NAME_SIZE];
        agent
            .save_name(&mut buffer.as_mut_slice())
            .expect("failed to save agent");
        assert_eq!(data, &buffer, "incorrect saved data");
    }
}
