use crate::{extract::Extract, CombatEvent, StateChange, TryExtract};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ArcDPS log error.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ErrorEvent {
    /// Error message.
    pub message: String,
}

impl Extract for ErrorEvent {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        let chars: [u8; 32] = transmute((
            event.time,
            event.src_agent,
            event.dst_agent,
            event.value,
            event.buff_dmg,
        ));

        Self {
            message: String::from_utf8_lossy(&chars)
                .trim_end_matches('\0')
                .into(),
        }
    }
}

impl TryExtract for ErrorEvent {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::Error
    }
}
