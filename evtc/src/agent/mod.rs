//! Agent bindings & utilities.
//!
//! Agents are the base used for nearly all entity types.

mod affinity;
mod agent_kind;
mod attack_target;
mod breakbar;
mod combat;
mod glider;
mod health;
mod id;
mod status;
mod targetable;
mod team;

#[cfg(feature = "realtime")]
pub mod realtime;

pub use self::{
    affinity::*, agent_kind::*, attack_target::*, breakbar::*, combat::*, glider::*, health::*,
    id::*, status::*, targetable::*, team::*,
};
