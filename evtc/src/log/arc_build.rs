use crate::{
    extract::{transmute_field, Extract},
    Event, StateChange, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ArcDPS log error.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArcBuildEvent {
    /// ArcDPS build string.
    pub build: String,
}

impl ArcBuildEvent {
    pub const MAX_LEN: usize = 32;
}

impl Extract for ArcBuildEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let bytes = transmute_field!(event.time as [u8; ArcBuildEvent::MAX_LEN]);

        Self {
            build: String::from_utf8_lossy(&bytes)
                .trim_end_matches('\0')
                .into(),
        }
    }
}

impl TryExtract for ArcBuildEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::Integrity
    }
}
