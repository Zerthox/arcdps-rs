use crate::{
    util::{read_string_buffer, Endian},
    Agent, Parse, ParseError, Skill,
};
use arcdps_evtc::CombatEvent;
use byteorder::ReadBytesExt;
use serde::{Deserialize, Serialize};
use std::io;

/// An EVTC log file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub header: Header,
    pub agents: Vec<Agent>,
    pub skills: Vec<Skill>,
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

/// An EVTC log header.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub date: String,
    pub revision: u8,
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
