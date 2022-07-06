use crate::util::get_str_from_pc_char;
use std::os::raw::c_char;

/// Represents an agent in a combat event.
///
/// ### Remarks
/// Names are available for the duration of the fight.
/// Due to this, this struct is not usable for longer than the function call.
/// If you need it for longer than that, consider converting it to [`AgentOwned`].
///
/// ```
/// let agent: AgentOwned = agent.into();
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Agent<'a> {
    pub name: Option<&'a str>,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}

impl From<&RawAgent> for Agent<'_> {
    fn from(ag: &RawAgent) -> Self {
        let name = unsafe { get_str_from_pc_char(ag.name) };
        Agent {
            name,
            id: ag.id,
            prof: ag.prof,
            elite: ag.elite,
            self_: ag.self_,
            team: ag.team,
        }
    }
}

/// An [`Agent`] with an owned [`String`].
/// For more info see [`Agent`].
#[derive(Debug, Clone)]
pub struct AgentOwned {
    pub name: Option<String>,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}

impl From<Agent<'_>> for AgentOwned {
    fn from(ag: Agent<'_>) -> Self {
        AgentOwned {
            name: ag.name.map(|x| x.to_string()),
            id: ag.id,
            prof: ag.prof,
            elite: ag.elite,
            self_: ag.self_,
            team: ag.team,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct RawAgent {
    pub name: *const c_char,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}
