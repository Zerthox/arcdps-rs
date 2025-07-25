//! Bindings for the ArcDPS EVTC API.
//!
//! Includes both types for Arc's realtime API used by plugins as well as Arc's log API consumed by parsers.
//!
//! ```no_run
//! use evtc::Event;
//!
//! fn total_damage_dealt(source: u64, target: u64, events: &[Event]) -> i32 {
//!     events
//!         .iter()
//!         .filter_map(|event| event.try_to_strike())
//!         .filter(|strike_event| {
//!             strike_event.strike.dealt_damage()
//!                 && strike_event.source.id == source
//!                 && strike_event.target.id == target
//!         })
//!         .map(|strike_event| strike_event.total_damage - strike_event.shield_damage as i32)
//!         .sum()
//! }
//! ```

pub mod agent;
pub mod buff;
pub mod content;
pub mod effect;
pub mod event;
pub mod extract;
mod game;
mod log;
pub mod marker;
pub mod missile;
pub mod player;
pub mod position;
mod ruleset;
pub mod skill;
mod state_change;
pub mod strike;
pub mod weapon;

pub use crate::{
    agent::{Affinity, AgentId, AgentKind},
    buff::{Attribute, BuffCategory, BuffCycle, BuffRemove},
    event::{Event, EventCategory, EventKind},
    extract::TryExtract,
    game::*,
    player::{Profession, Specialization},
    position::Position,
    ruleset::*,
    skill::{Activation, CustomSkill},
    state_change::*,
    strike::Strike,
};
