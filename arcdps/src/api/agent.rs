use crate::util::str_from_cstr;
use std::os::raw::c_char;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an agent in a combat event.
///
/// Names are available for the duration of the fight.
/// Due to this, this struct is not usable for longer than the function call.
/// If you need it for longer than that, consider converting it to [`AgentOwned`].
///
/// ```no_run
/// # use arcdps::{Agent, AgentOwned};
/// # let agent: arcdps::Agent = todo!();
/// let owned = agent.to_owned();
/// let owned: AgentOwned = agent.into();
/// ```
#[derive(Debug, Clone)]
pub struct Agent<'a> {
    /// Name of the agent.
    pub name: Option<&'a str>,

    /// Unique id.
    pub id: usize,

    /// Profession of the agent.
    pub prof: u32,

    /// Elite (specialization) of the agent.
    pub elite: u32,

    /// Whether the agent is self (the local player).
    pub is_self: u32,

    /// Team the agent is in.
    pub team: u16,
}

impl Agent<'_> {
    /// Converts the [`Agent`] to the owned version [`AgentOwned`].
    pub fn to_owned(self) -> AgentOwned {
        self.into()
    }
}

impl<'a> From<&'a RawAgent> for Agent<'a> {
    fn from(agent: &RawAgent) -> Self {
        Self {
            name: unsafe { str_from_cstr(agent.name) },
            id: agent.id,
            prof: agent.prof,
            elite: agent.elite,
            is_self: agent.is_self,
            team: agent.team,
        }
    }
}

/// [`Agent`] with an owned [`String`] name.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentOwned {
    /// Name of the agent.
    pub name: Option<String>,

    /// Unique id.
    pub id: usize,

    /// Profession of the agent.
    pub prof: u32,

    /// Elite (specialization) of the agent.
    pub elite: u32,

    /// Whether the agent is self (the local player).
    pub is_self: u32,

    /// Team the agent is in.
    pub team: u16,
}

impl From<Agent<'_>> for AgentOwned {
    fn from(agent: Agent<'_>) -> Self {
        Self {
            name: agent.name.map(|string| string.to_string()),
            id: agent.id,
            prof: agent.prof,
            elite: agent.elite,
            is_self: agent.is_self,
            team: agent.team,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawAgent {
    pub name: *const c_char,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub is_self: u32,
    pub team: u16,
}
