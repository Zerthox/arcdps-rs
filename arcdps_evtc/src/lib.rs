//! Bindings for the ArcDPS EVTC API.
//!
//! Includes everything shared between Arc's realtime API used by plugins and Arc's log API consumed by parsers.

pub mod agent;
pub mod breakbar;
pub mod buff;
pub mod effect;
pub mod event;
pub mod extract;
pub mod language;
pub mod log;
pub mod player;
pub mod position;
pub mod reward;
pub mod skill;
pub mod state_change;
pub mod strike;
pub mod weapon;

pub use crate::{
    agent::{Affinity, AgentId, AgentKind},
    buff::{Attribute, BuffCategory, BuffCycle, BuffRemove},
    event::{CombatEvent, EventCategory, EventKind, RawCombatEvent},
    extract::TryExtract,
    language::Language,
    player::{Profession, Specialization},
    position::Position,
    skill::{Activation, CustomSkill},
    state_change::StateChange,
    strike::Strike,
};
