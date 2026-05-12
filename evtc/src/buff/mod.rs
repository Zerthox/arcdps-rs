//! Buff bindings & utilities.
//!
//! Buffs are temporary "effects" applied to agents.
//! Some buffs modify the attributes of the destination agent.
//! They can be positive like Boons, negative like Conditions or mixed.
//! Some buffs do not do anything themselves and are simply used to as markers to track cooldowns, mechanics etc.

mod apply;
mod attribute;
mod change;
mod formula;
mod info;
mod initial;
mod remove;
mod stack;

pub use self::{
    apply::*, attribute::*, change::*, formula::*, info::*, initial::*, remove::*, stack::*,
};
