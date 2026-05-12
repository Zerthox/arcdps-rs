//! Bindings for the ArcDPS EVTC API.
//!
//! Includes both types for Arc's realtime API used by plugins as well as Arc's log API consumed by parsers.
//!
//! ```no_run
//! use evtc::event::{Event, CombatEvent};
//!
//! fn total_damage_dealt(source: u64, target: u64, events: &[Event]) -> i32 {
//!     events
//!         .iter()
//!         .filter_map(|event| event.try_extract::<CombatEvent>())
//!         .filter(|event| {
//!             event.result.is_strike_damage()
//!                 && event.source.id == source
//!                 && event.target.id == target
//!         })
//!         .map(|event| event.non_shield_strike_damage())
//!         .sum()
//! }
//! ```

pub mod legacy;

pub mod agent;
pub mod animation;
pub mod buff;
pub mod combat;
pub mod content;
pub mod effect;
pub mod event;
pub mod extract;
pub mod marker;
pub mod missile;
pub mod player;
pub mod position;
pub mod skill;
pub mod transformation;
pub mod weapon;
pub mod wvw;

mod game;
mod log;
mod ruleset;

pub use crate::{
    agent::{Affinity, AgentId, AgentKind},
    buff::{Attribute, BuffCategory},
    combat::{CombatEvent, CombatResult},
    event::{Event, EventKind, StateChange},
    extract::TryExtract,
    game::*,
    player::{Profession, Specialization},
    position::Position,
    ruleset::*,
    skill::CustomSkill,
};
