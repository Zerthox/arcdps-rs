use crate::{util::Endian, Agent, Header, LogTransformed, Parse, ParseError, Save, Skill};
use byteorder::{ReadBytesExt, WriteBytesExt};
use evtc::Event;
use std::{fs::File, io, path::Path};

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

    /// Every [`Event`] occurring in the log.
    ///
    /// Some events may also hold meta information, for example [`StateChange::BuffFormula`](crate::StateChange::BuffFormula).
    pub events: Vec<Event>,
}

impl Log {
    /// Parses a [`Log`] from a given [`Path`] to a log file.
    ///
    /// With the `"zevtc"` or `"zip"` feature enabled this also supports compressed log files.
    pub fn parse_file(path: impl AsRef<Path>) -> Result<Log, ParseError> {
        let path = path.as_ref();
        let mut file = io::BufReader::new(File::open(path)?);

        #[cfg(feature = "zevtc")]
        if let Some("zevtc" | "zip") = path.extension().and_then(|ext| ext.to_str()) {
            return Self::parse_zevtc(file);
        }

        Log::parse(&mut file)
    }

    /// Returns the [`Agent`] with the given id.
    #[inline]
    pub fn agent(&self, id: u64) -> Option<&Agent> {
        self.agents.iter().find(|agent| agent.id == id)
    }

    /// Returns a mutable reference to the [`Agent`] with the given id.
    #[inline]
    pub fn agent_mut(&mut self, id: u64) -> Option<&mut Agent> {
        self.agents.iter_mut().find(|agent| agent.id == id)
    }

    /// Returns the name(s) of the [`Agent`] with the given id.
    #[inline]
    pub fn agent_name(&self, id: u64) -> Option<&[String]> {
        self.agent(id).map(|agent| agent.name.as_slice())
    }

    /// Returns the [`Skill`] with the given id.
    #[inline]
    pub fn skill(&self, id: u32) -> Option<&Skill> {
        self.skills.iter().find(|skill| skill.id == id)
    }

    /// Returns a mutable reference to the [`Skill`] with the given id.
    #[inline]
    pub fn skill_mut(&mut self, id: u32) -> Option<&mut Skill> {
        self.skills.iter_mut().find(|skill| skill.id == id)
    }

    /// Returns the name of the [`Skill`] with the given id.
    #[inline]
    pub fn skill_name(&self, id: u32) -> Option<&str> {
        self.skill(id).map(|skill| skill.name.as_str())
    }

    /// Converts the log into its [`LogTransformed`] equivalent.
    #[inline]
    pub fn into_transformed(self) -> LogTransformed {
        self.into()
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
        while let Ok(event) = Event::parse(input) {
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
