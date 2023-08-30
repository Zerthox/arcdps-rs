//! ArcDPS API structs.

/// Extensions for live EVTC API.
#[path = "."]
mod ext {
    pub mod agent;
}

pub use arcdps_evtc::*;
pub use ext::agent::*;
