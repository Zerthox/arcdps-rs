//! Callback types.

use crate::{
    api::{Agent, CombatEvent, RawAgent, RawCombatEvent},
    imgui,
};
use std::{
    error::Error,
    os::raw::{c_char, c_void},
};
use windows::Win32::Foundation::{LPARAM, WPARAM};

/// Exported struct for ArcDPS plugins.
#[repr(C)]
pub struct ArcDpsExport {
    /// Size of exports table.
    pub size: usize,

    /// Unique plugin signature.
    ///
    /// Pick a random number that is not used by other modules.
    pub sig: u32,

    /// Imgui version number.
    ///
    /// Set to `18000` if you do not use Imgui (as of 2021-02-02).
    pub imgui_version: u32,

    /// Plugin name string.
    pub out_name: *const c_char,

    /// Plugin build (version) string.
    pub out_build: *const c_char,

    /// WndProc callback.
    ///
    /// Return is assigned to uMsg (return zero to not be processed by ArcDPS or game).
    pub wnd_nofilter: Option<RawWndprocCallback>,

    /// Combat callback.
    ///
    /// May be called asynchronously, use `id` to keep track of order.
    /// First event id will be `2`.
    /// Return is ignored.
    pub combat: Option<RawCombatCallback>,

    /// Imgui callback.
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
    pub wnd_filter: Option<RawWndprocCallback>,

    /// Options windows callback.
    ///
    /// Called once per window option checkbox in settings, with null at the end.
    /// Non-zero return disables ArcDPS drawing that checkbox.
    pub options_windows: Option<RawOptionsWindowsCallback>,
}

unsafe impl Sync for ArcDpsExport {}

pub type InitFunc = fn() -> Result<(), Box<dyn Error>>;

pub type ReleaseFunc = fn();

pub type RawWndprocCallback =
    unsafe extern "C" fn(h_wnd: *mut c_void, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> u32;
pub type WndProcCallback = fn(key: usize, key_down: bool, prev_key_down: bool) -> bool;

// TODO: should these be pointers?
pub type RawCombatCallback = unsafe extern "C" fn(
    ev: Option<&RawCombatEvent>,
    src: Option<&RawAgent>,
    dst: Option<&RawAgent>,
    skill_name: *mut c_char,
    id: u64,
    revision: u64,
);
pub type CombatCallback = fn(
    ev: Option<CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skill_name: Option<&'static str>,
    id: u64,
    revision: u64,
);

pub type RawImguiCallback = unsafe extern "C" fn(not_character_select_or_loading: u32);
pub type ImguiCallback = fn(ui: &imgui::Ui, not_character_select_or_loading: bool);

pub type RawOptionsCallback = unsafe extern "C" fn();
pub type OptionsCallback = fn(ui: &imgui::Ui);

pub type RawOptionsWindowsCallback = unsafe extern "C" fn(window_name: *mut c_char) -> bool;
pub type OptionsWindowsCallback = fn(ui: &imgui::Ui, window_name: Option<&str>) -> bool;
