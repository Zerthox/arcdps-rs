//! Missile bindings & utilities.
//!
//! Missiles are projectiles fired by combatants.

mod create;
mod launch;
mod remove;

pub use self::{create::*, launch::*, remove::*};
