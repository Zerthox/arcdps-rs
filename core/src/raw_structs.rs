use std::os::raw::{c_char, c_void};
use crate::helpers::get_str_from_pc_char;

pub type LPARAM = isize;
pub type LPVOID = *mut c_void;
pub type UINT = u32;
pub type WPARAM = usize;
// noinspection SpellCheckingInspection
pub type PCCHAR = *mut c_char;
pub type HWND = *mut c_void;


pub type RawWndprocCallback =
    unsafe fn(h_wnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> usize;
pub type RawCombatCallback = unsafe fn(
    ev: *mut CombatEvent,
    src: *mut RawAgent,
    dst: *mut RawAgent,
    skill_name: PCCHAR,
    id: u64,
    revision: u64,
);
pub type RawImguiCallback = unsafe fn(not_character_select_or_loading: u32);
pub type RawOptionsCallback = unsafe fn();
/// called once per 'window' option checkbox, with null at the end, non-zero return disables arcdps drawing that checkbox
pub type RawOptionsWindowsCallback = unsafe fn(window_name: PCCHAR) -> bool;

/// Gets called on `get_init_address`.
pub type InitFunc = fn();
/// Gets called on `unload`.
pub type ReleaseFunc = fn();

/// Provides a [imgui::Ui] object that is needed to draw anything.
/// The second parameter is true whenever the player is __not__ in character select, loading screens or forced cameras.
pub type ImguiCallback = fn(ui: &imgui::Ui, not_character_select_or_loading: bool);
/// Provides a [imgui::Ui] object that is needed to draw anything.
pub type OptionsCallback = fn(ui: &imgui::Ui);
/// Called per window option checkbox. Does not draw the checkbox if returned true.
pub type OptionsWindowsCallback = fn(window_name: &str) -> bool;
/// Provides safe abstractions for the combat event.
pub type CombatCallback = fn(
    ev: Option<&CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skill_name: Option<&'static str>,
    id: u64,
    revision: u64,
);

impl From<*mut RawAgent> for Agent<'_> {
    fn from(ag: *mut RawAgent) -> Self {
        let ag = unsafe { *ag };
        let name = unsafe { get_str_from_pc_char(ag.name) };
        Agent {
            name,
            id: ag.id,
            prof: ag.prof,
            elite: ag.elite,
            self_: ag.self_,
            team: ag.team,
        }
    }
}

/// Represents an agent in a combat event.
/// ### Remarks
/// Names are available for the duration of the fight.
/// Due to this, this struct is not usable for longer than the function call.
/// If you need it for longer than that, consider converting it to [`AgentOwned`].
/// ```
/// use arcdps::*;
/// let agent: AgentOwned = agent.into();
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Agent<'a> {
    pub name: Option<&'a str>,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}

/// An [`Agent`] with an owned [`String`].
/// For more info see [`Agent`].
#[derive(Debug, Clone)]
pub struct AgentOwned {
    pub name: Option<String>,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}

impl From<Agent<'_>> for AgentOwned {
    fn from(ag: Agent<'_>) -> Self {
        AgentOwned {
            name: ag.name.map(|x| x.to_string()),
            id: ag.id,
            prof: ag.prof,
            elite: ag.elite,
            self_: ag.self_,
            team: ag.team,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CombatEvent {
    pub time: u64,
    pub src_agent: usize,
    pub dst_agent: usize,
    pub value: i32,
    pub buff_dmg: i32,
    pub overstack_value: u32,
    pub skill_id: u32,
    pub src_instance_id: u16,
    pub dst_instance_id: u16,
    pub src_master_instance_id: u16,
    pub dst_master_instance_id: u16,
    pub iff: u8,
    pub buff: u8,
    pub result: u8,
    pub is_activation: u8,
    pub is_buff_remove: u8,
    pub is_ninety: u8,
    pub is_fifty: u8,
    pub is_moving: u8,
    pub is_statechange: u8,
    pub is_flanking: u8,
    pub is_shields: u8,
    pub is_off_cycle: u8,
    pub pad61: u8,
    pub pad62: u8,
    pub pad63: u8,
    pub pad64: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawAgent {
    pub name: PCCHAR,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}

// noinspection SpellCheckingInspection
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArcDpsExport {
    pub size: usize,
    pub sig: u32,
    pub imgui_version: u32,
    pub out_name: *const u8,
    pub out_build: *const u8,
    pub wnd_nofilter: Option<RawWndprocCallback>,
    pub combat: Option<RawCombatCallback>,
    pub imgui: Option<RawImguiCallback>,
    pub options_end: Option<RawOptionsCallback>,
    pub combat_local: Option<RawCombatCallback>,
    pub wnd_filter: Option<RawWndprocCallback>,
    pub options_windows: Option<RawOptionsWindowsCallback>,
}

unsafe impl Send for ArcDpsExport {}
unsafe impl Sync for ArcDpsExport {}
