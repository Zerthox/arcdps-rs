use crate::{
    util::{read_string_buffer, write_string_buffer, Endian},
    Agent, Parse, ParseError, Save, Skill,
};
use arcdps_evtc::CombatEvent;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An EVTC log.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Log {
    /// The log header with meta information.
    pub header: Header,

    /// Agents (entities) present in the log.
    pub agents: Vec<Agent>,

    /// Information about skills used in the log.
    pub skills: Vec<Skill>,

    /// Every [`CombatEvent`] occurring in the log.
    ///
    /// Some events may also hold meta information, for example [`StateChange::BuffFormula`](crate::StateChange::BuffFormula).
    pub events: Vec<CombatEvent>,
}

impl Log {
    #[inline]
    pub fn agent(&self, address: u64) -> Option<&Agent> {
        self.agents.iter().find(|agent| agent.address == address)
    }

    #[inline]
    pub fn agent_mut(&mut self, address: u64) -> Option<&mut Agent> {
        self.agents
            .iter_mut()
            .find(|agent| agent.address == address)
    }

    #[inline]
    pub fn agent_name(&self, address: u64) -> Option<&[String]> {
        self.agent(address).map(|agent| agent.name.as_slice())
    }

    #[inline]
    pub fn skill(&self, id: u32) -> Option<&Skill> {
        self.skills.iter().find(|skill| skill.id == id)
    }

    #[inline]
    pub fn skill_mut(&mut self, id: u32) -> Option<&mut Skill> {
        self.skills.iter_mut().find(|skill| skill.id == id)
    }

    #[inline]
    pub fn skill_name(&self, id: u32) -> Option<&str> {
        self.skill(id).map(|skill| skill.name.as_str())
    }
}

impl Parse for Log {
    type Error = ParseError;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        let header = Header::parse(input)?;

        // we only support current revision
        if header.revision != 1 {
            return Err(ParseError::UnsupportedRevision(header.revision));
        }

        let agent_count = input.read_u32::<Endian>()?;
        let agents = Agent::parse_multi(input, agent_count as usize)?;

        let skill_count = input.read_u32::<Endian>()?;
        let skills = Skill::parse_multi(input, skill_count as usize)?;

        let mut events = Vec::new();
        while let Ok(event) = CombatEvent::parse(input) {
            events.push(event);
        }

        Ok(Self {
            header,
            agents,
            skills,
            events,
        })
    }
}

impl Save for Log {
    type Error = io::Error;

    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error> {
        self.header.save(output)?;

        output.write_u32::<Endian>(self.agents.len() as u32)?;
        for agent in &self.agents {
            agent.save(output)?;
        }

        output.write_u32::<Endian>(self.skills.len() as u32)?;
        for skill in &self.skills {
            skill.save(output)?;
        }

        for event in &self.events {
            event.save(output)?;
        }

        Ok(())
    }
}

/// An EVTC log header.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Header {
    /// Date this log was recorded.
    pub date: String,

    /// EVTC API revision used.
    pub revision: u8,

    /// Boss id of the log target.
    ///
    /// An id of `1` indicates a WvW log.
    /// An id of `2` indicates a map log.
    pub boss_id: u16,
}

impl Header {
    /// Size of the date string.
    pub const DATE_SIZE: usize = 12;
}

impl Parse for Header {
    type Error = ParseError;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        let evtc = read_string_buffer::<4>(input)?;
        if evtc != "EVTC" {
            return Err(ParseError::NotEvtc);
        }

        let date = read_string_buffer::<{ Self::DATE_SIZE - 4 }>(input)?;
        let revision = input.read_u8()?;
        let boss_id = input.read_u16::<Endian>()?;

        // unused byte in arc header
        input.read_u8()?;

        Ok(Self {
            date: evtc + &date,
            revision,
            boss_id,
        })
    }
}

impl Save for Header {
    type Error = io::Error;

    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error> {
        write_string_buffer::<{ Self::DATE_SIZE }>(output, &self.date)?;
        output.write_u8(self.revision)?;
        output.write_u16::<Endian>(self.boss_id)?;

        // unused byte
        output.write_u8(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header() {
        let header = Header {
            date: "EVTC20230328".into(),
            revision: 1,
            boss_id: 123,
        };

        let mut vec = Vec::with_capacity(64);
        header.save(&mut vec).unwrap();

        let parsed = Header::parse(&mut vec.as_slice()).unwrap();
        assert_eq!(parsed, header);
    }
}