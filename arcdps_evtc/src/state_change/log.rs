use crate::{CombatEvent, Extract};
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
    unsafe fn extract(event: &CombatEvent) -> Self {
        Self {
            time: event.time,
            server_time: transmute(event.value),
            local_time: transmute(event.buff_dmg),
            id: event.src_agent,
        }
    }
}
