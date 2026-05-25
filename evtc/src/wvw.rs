use crate::{
    Event, StateChange, TryExtract,
    extract::{Extract, transmute_field},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// WvW teams information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WvwTeams {
    pub time: u64,
    pub red_shard: u32,
    pub blue_shard: u32,
    pub green_shard: u32,
    pub red_team: u32,
    pub blue_team: u32,
    pub green_team: u32,
}

impl Extract for WvwTeams {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let [
            red_shard,
            blue_shard,
            green_shard,
            red_team,
            blue_team,
            green_team,
        ] = transmute_field!(event.src_agent as [u32; 6]);
        Self {
            time: event.time,
            red_shard,
            blue_shard,
            green_shard,
            red_team,
            blue_team,
            green_team,
        }
    }
}

impl TryExtract for WvwTeams {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::WvwTeams
    }
}

/// WvW objective status.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WvwObjectiveStatus {
    pub time: u64,
    pub map_id: i32,
    pub team_id: i32,
    pub objective_id: u32,
    pub objective_type: u8,
    pub upgrade_progress_count: u32,
}

impl Extract for WvwObjectiveStatus {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            map_id: event.value,
            team_id: event.buff_dmg,
            objective_id: event.skill_id,
            objective_type: event.buff,
            upgrade_progress_count: event.get_pad_id(),
        }
    }
}

impl TryExtract for WvwObjectiveStatus {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::WvwObjectiveStatus
    }
}
