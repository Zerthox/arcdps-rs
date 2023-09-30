//! Bindings for the ArcDPS EVTC API.
//!
//! Includes both types for Arc's realtime API used by plugins as well as Arc's log API consumed by parsers.

pub mod agent;
pub mod buff;
pub mod effect;
pub mod event;
pub mod extract;
mod game;
mod log;
pub mod player;
pub mod position;
pub mod skill;
mod state_change;
pub mod strike;
pub mod weapon;

#[cfg(feature = "serde")]
mod serde_hex;

pub use crate::{
    agent::{Affinity, AgentId, AgentKind},
    buff::{Attribute, BuffCategory, BuffCycle, BuffRemove},
    event::{Event, EventCategory, EventKind},
    extract::TryExtract,
    game::Language,
    player::{Profession, Specialization},
    position::Position,
    skill::{Activation, CustomSkill},
    state_change::StateChange,
    strike::Strike,
};
