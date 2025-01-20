//! Realtime API [`Agent`] types.

use super::AgentKind;
use std::{ffi::CStr, os::raw::c_char};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an agent in a realtime combat event.
///
/// Names are available for the duration of the fight.
/// Due to this, this struct is not usable for longer than the function call.
/// If you need it for longer than its lifetime, consider converting it to [`AgentOwned`].
///
/// ```no_run
/// # use evtc::agent::realtime::{Agent, AgentOwned};
/// # let agent: &Agent = todo!();
/// let owned = agent.to_owned();
/// let owned: AgentOwned = agent.into();
/// ```
#[derive(Debug)]
pub struct Agent {
    /// Name of the agent.
    name: *const c_char,

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

impl Agent {
    /// Returns the agent's name.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        if !self.name.is_null() {
            unsafe { CStr::from_ptr(self.name) }.to_str().ok()
        } else {
            None
        }
    }

    /// Returns the raw pointer to the agent's name.
    #[inline]
    pub fn name_ptr(&self) -> *const c_char {
        self.name
    }

    /// Converts the [`Agent`] to the owned version [`AgentOwned`].
    #[inline]
    pub fn to_owned(&self) -> AgentOwned {
        self.into()
    }

    /// Determines the kind of agent.
    #[inline]
    pub const fn kind(&self) -> AgentKind {
        AgentKind::new(self.prof, self.elite)
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

impl From<Agent> for AgentOwned {
    #[inline]
    fn from(agent: Agent) -> Self {
        (&agent).into()
    }
}

impl From<&Agent> for AgentOwned {
    #[inline]
    fn from(agent: &Agent) -> Self {
        Self {
            name: agent.name().map(|string| string.to_owned()),
            id: agent.id,
            prof: agent.prof,
            elite: agent.elite,
            is_self: agent.is_self,
            team: agent.team,
        }
    }
}
