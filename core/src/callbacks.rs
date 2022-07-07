//! Callback types.

use crate::{
    api::{Agent, CombatEvent, RawAgent},
    extras::callbacks::{
        ExtrasInitFunc, ExtrasSquadUpdateCallback, RawExtrasSubscriberInit, RawSquadUpdateCallback,
    },
    imgui,
};
use std::os::raw::{c_char, c_void};
use windows::Win32::Foundation::{LPARAM, WPARAM};

// TODO: should any of this be moved somewhere else?

/// Reference on what fields are currently supported by the [`arcdps_export!`](arcdps_codegen::arcdps_export) macro.
/// This struct is not used anywhere.
pub struct SupportedFields {
    pub name: &'static str,
    pub sig: u32,
    pub init: Option<InitFunc>,
    pub release: Option<ReleaseFunc>,
    pub raw_wnd_nofilter: Option<RawWndprocCallback>,
    pub raw_imgui: Option<RawImguiCallback>,
    pub raw_options_end: Option<RawOptionsCallback>,
    pub raw_combat: Option<RawCombatCallback>,
    pub raw_wnd_filter: Option<RawWndprocCallback>,
    pub raw_options_windows: Option<RawOptionsWindowsCallback>,
    pub raw_combat_local: Option<RawCombatCallback>,
    pub raw_unofficial_extras_init: Option<RawExtrasSubscriberInit>,
    pub raw_unofficial_extras_squad_update: Option<RawSquadUpdateCallback>,
    pub wnd_nofilter: Option<WndProcCallback>,
    pub combat: Option<CombatCallback>,
    pub imgui: Option<ImguiCallback>,
    pub options_end: Option<OptionsCallback>,
    pub combat_local: Option<CombatCallback>,
    pub wnd_filter: Option<WndProcCallback>,
    pub options_windows: Option<OptionsWindowsCallback>,
    pub unofficial_extras_init: Option<ExtrasInitFunc>,
    pub unofficial_extras_squad_update: Option<ExtrasSquadUpdateCallback>,
}

#[repr(C)]
pub struct ArcDpsExport {
    pub size: usize,
    pub sig: u32,
    pub imgui_version: u32,
    pub out_name: *const c_char,
    pub out_build: *const c_char,
    pub wnd_nofilter: Option<RawWndprocCallback>,
    pub combat: Option<RawCombatCallback>,
    pub imgui: Option<RawImguiCallback>,
    pub options_end: Option<RawOptionsCallback>,
    pub combat_local: Option<RawCombatCallback>,
    pub wnd_filter: Option<RawWndprocCallback>,
    pub options_windows: Option<RawOptionsWindowsCallback>,
}

unsafe impl Sync for ArcDpsExport {}

pub type RawWndprocCallback =
    unsafe extern "C" fn(h_wnd: *mut c_void, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> u32;

// TODO: should these be pointers?
pub type RawCombatCallback = unsafe extern "C" fn(
    ev: Option<&CombatEvent>,
    src: Option<&RawAgent>,
    dst: Option<&RawAgent>,
    skill_name: *mut c_char,
    id: u64,
    revision: u64,
);

pub type RawImguiCallback = unsafe extern "C" fn(not_character_select_or_loading: u32);

pub type RawOptionsCallback = unsafe extern "C" fn();

/// called once per 'window' option checkbox, with null at the end, non-zero
/// return disables arcdps drawing that checkbox
pub type RawOptionsWindowsCallback = unsafe extern "C" fn(window_name: *mut c_char) -> bool;

/// Gets called on load.
pub type InitFunc = fn() -> Result<(), Box<dyn std::error::Error>>;

/// Gets called on unload.
pub type ReleaseFunc = fn();

/// Gets called for each key pressed. Returning true will allow arcdps and Gw2 to receive the key press.
/// First parameter indicates the virtual key code (<https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes>),
/// second parameter is true if the key was pressed and false when released,
/// third parameter is true if the key was down before this event occurred, for example by holding it down.
pub type WndProcCallback = fn(key: usize, key_down: bool, prev_key_down: bool) -> bool;

/// Provides a [imgui::Ui] object that is needed to draw anything.
/// The second parameter is true whenever the player is __not__ in character select, loading screens or forced cameras.
pub type ImguiCallback = fn(ui: &imgui::Ui, not_character_select_or_loading: bool);

/// Provides a [imgui::Ui] object that is needed to draw anything.
pub type OptionsCallback = fn(ui: &imgui::Ui);

/// Called per window option checkbox. Does not draw the checkbox if returned true.
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
