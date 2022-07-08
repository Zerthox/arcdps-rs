//! Callback types.

use crate::{
    api::{Agent, CombatEvent, RawAgent},
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

/// Callback for plugin load.
pub type InitFunc = fn() -> Result<(), Box<dyn Error>>;

/// Callback for plugin unload.
pub type ReleaseFunc = fn();

pub type RawWndprocCallback =
    unsafe extern "C" fn(h_wnd: *mut c_void, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> u32;

/// Callback for key presses.
///
/// Returning `true` will allow ArcDPS and GW2 to receive the key press.
/// First parameter indicates the virtual key code (<https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes>),
/// second parameter is `true` if the key was pressed and `false` when released,
/// third parameter is `true` if the key was down before this event occurred, for example by holding it down.
pub type WndProcCallback = fn(key: usize, key_down: bool, prev_key_down: bool) -> bool;

// TODO: should these be pointers?
pub type RawCombatCallback = unsafe extern "C" fn(
    ev: Option<&CombatEvent>,
    src: Option<&RawAgent>,
    dst: Option<&RawAgent>,
    skill_name: *mut c_char,
    id: u64,
    revision: u64,
);

/// Callback for combat events.
///
/// This is the same signature for both area as well as local events.
pub type CombatCallback = fn(
    ev: Option<&CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skill_name: Option<&'static str>,
    id: u64,
    revision: u64,
);

pub type RawImguiCallback = unsafe extern "C" fn(not_character_select_or_loading: u32);

/// Callback for standalone UI creation.
///
/// Provides a [imgui::Ui] object that is needed to draw anything.
/// The second parameter is `true` whenever the player is **not** in character select, loading screens or forced cameras.
pub type ImguiCallback = fn(ui: &imgui::Ui, not_character_select_or_loading: bool);

pub type RawOptionsCallback = unsafe extern "C" fn();

/// Callback for plugin settings UI creation.
///
/// Provides a [imgui::Ui] object that is needed to draw anything.
pub type OptionsCallback = fn(ui: &imgui::Ui);

pub type RawOptionsWindowsCallback = unsafe extern "C" fn(window_name: *mut c_char) -> bool;

/// Callback for window options.
///
/// Called for each window checkbox in ArcDPS settings.
/// Last call will always be with [`None`].
/// Does not draw the checkbox if returning `true`.
pub type OptionsWindowsCallback = fn(ui: &imgui::Ui, window_name: Option<&str>) -> bool;
