#![allow(clippy::upper_case_acronyms)]

use crate::{helpers::get_str_from_pc_char, imgui};
use std::os::raw::{c_char, c_void};

pub type LPARAM = isize;
pub type LPVOID = *mut c_void;
pub type UINT = u32;
pub type WPARAM = usize;
pub type PCCHAR = *mut c_char;
pub type LPCSTR = *const c_char;
pub type HWND = *mut c_void;
pub type HMODULE = *mut c_void;
pub type HANDLE = *mut std::os::raw::c_void;

pub const WM_KEYDOWN: u32 = 0x100;
pub const WM_KEYUP: u32 = 0x101;
pub const WM_SYSKEYDOWN: u32 = 0x104;
pub const WM_SYSKEYUP: u32 = 0x105;

extern "system" {
    pub fn GetProcAddress(module: HMODULE, proc_name: LPCSTR) -> *mut c_void;
}

pub type RawWndprocCallback =
    unsafe extern "C" fn(h_wnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> UINT;
pub type RawCombatCallback = unsafe extern "C" fn(
    ev: Option<&CombatEvent>,
    src: Option<&RawAgent>,
    dst: Option<&RawAgent>,
    skill_name: PCCHAR,
    id: u64,
    revision: u64,
);
pub type RawImguiCallback = unsafe extern "C" fn(not_character_select_or_loading: u32);
pub type RawOptionsCallback = unsafe extern "C" fn();
/// called once per 'window' option checkbox, with null at the end, non-zero
/// return disables arcdps drawing that checkbox
pub type RawOptionsWindowsCallback = unsafe extern "C" fn(window_name: PCCHAR) -> bool;

/// Gets called on load.
pub type InitFunc = fn() -> Result<(), Box<dyn std::error::Error>>;
/// Gets called on unload.
pub type ReleaseFunc = fn();

/// Gets called for each key pressed. Returning true will allow arcdps and Gw2
/// to receive the key press. first parameter indicates the virtual key code (<https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes>)
/// second parameter is true if the key was pressed and false when released
/// third parameter is true if the key was down before this event occured, for
/// example by holding it down
pub type WndProcCallback = fn(key: usize, key_down: bool, prev_key_down: bool) -> bool;
/// Provides a [imgui::Ui] object that is needed to draw anything.
/// The second parameter is true whenever the player is __not__ in character
/// select, loading screens or forced cameras.
pub type ImguiCallback = fn(ui: &imgui::Ui, not_character_select_or_loading: bool);
/// Provides a [imgui::Ui] object that is needed to draw anything.
pub type OptionsCallback = fn(ui: &imgui::Ui);
/// Called per window option checkbox. Does not draw the checkbox if returned
/// true.
pub type OptionsWindowsCallback = fn(ui: &imgui::Ui, window_name: Option<&str>) -> bool;
/// Provides safe abstractions for the combat event.
pub type CombatCallback = fn(
    ev: Option<&CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skill_name: Option<&'static str>,
    id: u64,
    revision: u64,
);

pub type Export0 = unsafe extern "C" fn() -> *mut u16;
pub type Export3 = unsafe extern "C" fn(*mut u8);
pub type Export5 = unsafe extern "C" fn(*mut [*mut imgui::sys::ImVec4; 5]);
pub type Export6 = unsafe extern "C" fn() -> u64;
pub type Export7 = unsafe extern "C" fn() -> u64;
pub type Export8 = Export3;
pub type Export9 = unsafe extern "C" fn(&CombatEvent, u32);

impl From<&RawAgent> for Agent<'_> {
    fn from(ag: &RawAgent) -> Self {
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
/// If you need it for longer than that, consider converting it to
/// [`AgentOwned`].
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

unsafe impl Sync for ArcDpsExport {}
