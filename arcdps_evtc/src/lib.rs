//! Bindings for the ArcDPS EVTC API.
//!
//! Includes everything shared between Arc's realtime API used by plugins and Arc's log API consumed by parsers.

pub mod agent;
pub mod breakbar;
pub mod buff;
pub mod effect;
pub mod event;
pub mod language;
pub mod log;
pub mod player;
pub mod position;
pub mod skill;
pub mod state_change;
pub mod strike;

pub use crate::{
    agent::{Affinity, AgentId, AgentKind},
    buff::{Attribute, BuffCategory, BuffCycle, BuffRemove},
    event::{CombatEvent, EventCategory, EventKind, RawCombatEvent},
    language::Language,
    player::{Profession, Specialization},
    position::Position,
    skill::{Activation, CustomSkill},
    state_change::StateChange,
    strike::Strike,
};

/// Extracts information from a combat event.
pub trait Extract {
    /// Extracts [`Self`] from the combat event.
    ///
    /// # Safety
    /// This is safe when the given event is a valid event to extract [`Self`] from.
    unsafe fn extract(event: &CombatEvent) -> Self;
}
