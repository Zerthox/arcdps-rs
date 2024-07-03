mod arc_build;
mod error;

pub use self::{arc_build::*, error::*};

use crate::{extract::Extract, Event, StateChange, TryExtract};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Log started, ended or target changed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LogEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Server Unix timestamp.
    pub server_time: u32,

    /// Local Unix timestamp.
    pub local_time: u32,

    /// Target species id (or ArcDPS id `0x637261`).
    pub id: u64,
}

impl Extract for LogEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            server_time: transmute::<i32, u32>(event.value),
            local_time: transmute::<i32, u32>(event.buff_dmg),
            id: event.src_agent,
        }
    }
}

impl TryExtract for LogEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        matches!(
            event.get_statechange(),
            StateChange::SquadCombatStart | StateChange::SquadCombatEnd | StateChange::LogNPCUpdate
        )
    }
}
