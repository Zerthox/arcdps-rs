//! Buff bindings & utilities.
//!
//! Buffs are temporary "effects" applied to agents.
//! Some buffs modify the attributes of the destination agent.
//! They can be positive like Boons, negative like Conditions or mixed.
//! Some buffs do not do anything themselves and are simply used to as markers to track cooldowns, mechanics etc.

mod apply;
mod attribute;
mod damage;
mod formula;
mod info;
mod initial;
mod remove;
mod stack;

pub use self::apply::*;
pub use self::attribute::*;
pub use self::damage::*;
pub use self::formula::*;
pub use self::info::*;
pub use self::initial::*;
pub use self::remove::*;
pub use self::stack::*;
