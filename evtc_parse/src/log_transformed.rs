use crate::{Agent, EventKind, Header, Log, Parse, ParseError, Skill};
use std::io;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A transformed EVTC log.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LogTransformed {
    /// The log header with meta information.
    pub header: Header,

    /// Agents (entities) present in the log.
    pub agents: Vec<Agent>,

    /// Information about skills used in the log.
    pub skills: Vec<Skill>,

    /// Every [`Event`] occurring in the log transformed as [`EventKind`].
    pub events: Vec<EventKind>,
}

impl LogTransformed {
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

impl From<Log> for LogTransformed {
    #[inline]
    fn from(log: Log) -> Self {
        Self {
            header: log.header,
            agents: log.agents,
            skills: log.skills,
            events: log
                .events
                .into_iter()
                .map(|event| event.into_kind())
                .collect(),
        }
    }
}

impl Parse for LogTransformed {
    type Error = ParseError;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        Log::parse(input).map(Into::into)
    }
}
