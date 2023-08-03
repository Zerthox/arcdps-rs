//! Bindings for the ArcDPS EVTC API.
//!
//! Includes everything shared between Arc's realtime API used by plugins and Arc's log API consumed by parsers.

mod agent;
mod breakbar;
mod buff;
mod effect;
mod event;
mod language;
mod player;
mod position;
mod skill;
mod state_change;
mod strike;

pub use self::agent::*;
pub use self::breakbar::*;
pub use self::buff::*;
pub use self::effect::*;
pub use self::event::*;
pub use self::language::*;
pub use self::player::*;
pub use self::position::*;
pub use self::skill::*;
pub use self::state_change::*;
pub use self::strike::*;

/// Extracts information from a combat event.
pub trait Extract {
    /// Extracts [`Self`] from the combat event.
    ///
    /// # Safety
    /// This is safe when the given event is a valid event to extract [`Self`] from.
    unsafe fn extract(event: &CombatEvent) -> Self;
}
