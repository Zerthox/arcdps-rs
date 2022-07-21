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
/// ```ignore
/// let agent: AgentOwned = agent.into();
/// ```
#[derive(Debug, Clone)]
pub struct Agent<'a> {
    pub name: Option<&'a str>,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub is_self: u32,
    pub team: u16,
}

impl From<RawAgent> for Agent<'_> {
    fn from(agent: RawAgent) -> Self {
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

/// An [`Agent`] with an owned [`String`].
/// For more info see [`Agent`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentOwned {
    pub name: Option<String>,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub is_self: u32,
    pub team: u16,
}

impl From<Agent<'_>> for AgentOwned {
    fn from(agent: Agent<'_>) -> Self {
        Self {
            name: agent.name.map(|x| x.to_string()),
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
