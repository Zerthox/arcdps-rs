use crate::{
    util::{read_string_buffer, Endian},
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
#[derive(Debug, Clone)]
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

impl Parse for Header {
    type Error = ParseError;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        let date = read_string_buffer::<12>(input)?;
        let revision = input.read_u8()?;
        let boss_id = input.read_u16::<Endian>()?;

        // unused byte in arc header
        input.read_u8()?;

        Ok(Self {
            date,
            revision,
            boss_id,
        })
    }
}

impl Save for Header {
    type Error = io::Error;

    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error> {
        output.write_all(self.date.as_bytes())?;
        output.write_u8(self.revision)?;
        output.write_u16::<Endian>(self.boss_id)
    }
}
