//! Callback types.

use crate::{
    evtc::{Agent, CombatEvent, RawAgent, RawCombatEvent},
    imgui,
    util::abi,
};
use std::os::raw::c_char;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};

/// Exported struct for ArcDPS plugins.
#[repr(C)]
pub struct ArcDpsExport {
    /// Size of exports table.
    pub size: usize,

    /// Unique plugin signature.
    ///
    /// Pick a random number that is not used by other modules.
    pub sig: u32,

    /// ImGui version number.
    ///
    /// Set to `18000` if you do not use ImGui (as of 2021-02-02).
    pub imgui_version: u32,

    /// Plugin name string.
    pub out_name: *const c_char,

    /// Plugin build (version) string.
    pub out_build: *const c_char,

    /// WndProc callback.
    ///
    /// Return is assigned to uMsg (return zero to not be processed by ArcDPS or game).
    pub wnd_nofilter: Option<RawWndProcCallback>,

    /// Combat callback.
    ///
    /// May be called asynchronously, use `id` to keep track of order.
    /// First event id will be `2`.
    /// Return is ignored.
    pub combat: Option<RawCombatCallback>,

    /// ImGui callback.
    pub imgui: Option<RawImguiCallback>,

    /// Options callback.
    ///
    /// For a plugin options tab.
    pub options_end: Option<RawOptionsCallback>,

    /// Local combat callback.
    ///
    /// Like `combat` (area) but from chat log.
    pub combat_local: Option<RawCombatCallback>,

    /// Filtered WndProc callback.
    ///
    /// Like `wnd_nofilter` but input fitlered using modifiers.
    pub wnd_filter: Option<RawWndProcCallback>,

    /// Options windows callback.
    ///
    /// Called once per window option checkbox in settings, with null at the end.
    /// Non-zero return disables ArcDPS drawing that checkbox.
    pub options_windows: Option<RawOptionsWindowsCallback>,
}

unsafe impl Sync for ArcDpsExport {}

pub type InitFunc = fn() -> Result<(), String>;

pub type ReleaseFunc = fn();

pub type UpdateUrlFunc = fn() -> Option<String>;

pub type WndProcCallback = fn(key: usize, key_down: bool, prev_key_down: bool) -> bool;

pub type CombatCallback = fn(
    event: Option<CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skill_name: Option<&'static str>,
    id: u64,
    revision: u64,
);

pub type ImguiCallback = fn(ui: &imgui::Ui, not_character_select_or_loading: bool);

pub type OptionsCallback = fn(ui: &imgui::Ui);

pub type OptionsWindowsCallback = fn(ui: &imgui::Ui, window_name: Option<&str>) -> bool;

abi! {
    pub type RawWndProcCallback =
        unsafe extern fn(h_wnd: HWND, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> u32;

    pub type RawCombatCallback = unsafe extern fn(
        event: *const RawCombatEvent,
        src: *const RawAgent,
        dst: *const RawAgent,
        skill_name: *const c_char,
        id: u64,
        revision: u64,
    );

    pub type RawImguiCallback = unsafe extern fn(not_character_select_or_loading: u32);

    pub type RawOptionsCallback = unsafe extern fn();

    pub type RawOptionsWindowsCallback = unsafe extern fn(window_name: *const c_char) -> bool;
}
