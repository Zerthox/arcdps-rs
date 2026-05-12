use crate::{Agent, Event, EventKind, Header, Log, Parse, ParseError, Skill};
use std::io;

use evtc::legacy::LegacyEventKind;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A transformed EVTC log.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LogTransformed<T = EventKind> {
    /// The log header with meta information.
    pub header: Header,

    /// Agents (entities) present in the log.
    pub agents: Vec<Agent>,

    /// Information about skills used in the log.
    pub skills: Vec<Skill>,

    /// Every [`Event`](crate::Event) occurring in the log transformed.
    pub events: Vec<T>,
}

/// A transformed EVTC log with legacy events.
pub type LogTransformedLegacy = LogTransformed<LegacyEventKind>;

impl LogTransformed {
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

    /// Returns the name of the [`Agent`] with the given id.
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
}

impl<T> From<Log> for LogTransformed<T>
where
    T: From<Event>,
{
    #[inline]
    fn from(log: Log) -> Self {
        Self {
            header: log.header,
            agents: log.agents,
            skills: log.skills,
            events: log.events.into_iter().map(|event| event.into()).collect(),
        }
    }
}

impl<T> Parse for LogTransformed<T>
where
    T: From<Event>,
{
    type Error = ParseError;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        Log::parse(input).map(Into::into)
    }
}
