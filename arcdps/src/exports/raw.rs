//! Raw ArcDPS exports.

use crate::{api::RawCombatEvent, globals::ARC_GLOBALS, imgui::sys::ImVec4};
use std::os::raw::c_char;

pub type Export0 = unsafe extern "C" fn() -> *const u16;
pub type Export3 = unsafe extern "C" fn(string: *const c_char);
pub type Export5 = unsafe extern "C" fn(out: *mut [*mut ImVec4; 5]);
pub type Export6 = unsafe extern "C" fn() -> u64;
pub type Export7 = unsafe extern "C" fn() -> u64;
pub type Export8 = unsafe extern "C" fn(string: *const c_char);
pub type Export9 = unsafe extern "C" fn(event: *const RawCombatEvent, sig: u32);

/// Retrieves path to ArcDPS ini config file as wide char string.
#[inline]
pub unsafe fn e0_config_path() -> *const u16 {
    (ARC_GLOBALS.e0.expect("failed to find arc export e0"))()
}

/// Logs a string to `arcdps.log` file.
#[inline]
pub unsafe fn e3_log_file(string: *const c_char) {
    (ARC_GLOBALS.e3.expect("failed to find arc export e3"))(string)
}

/// Writes color array pointers to buffer.
#[inline]
pub unsafe fn e5_colors(buffer: *mut [*mut ImVec4; 5]) {
    (ARC_GLOBALS.e5.expect("failed to find arc export e5"))(buffer)
}

/// Retrieves bit mask of current ArcDPS UI settings.
#[inline]
pub unsafe fn e6_ui_settings() -> u64 {
    (ARC_GLOBALS.e6.expect("failed to find arc export e6"))()
}

/// Retrieves modifier keys as virtual key codes.
#[inline]
pub unsafe fn e7_modifiers() -> u64 {
    (ARC_GLOBALS.e7.expect("failed to find arc export e7"))()
}

/// Logs a string to the ArcDPS logger window.
///
/// Colors are HTML-like: `<c=#aaaaaa>colored text</c>`.
#[inline]
pub unsafe fn e8_log_window(string: *const c_char) {
    (ARC_GLOBALS.e8.expect("failed to find arc export e8"))(string)
}

/// Adds a [`RawCombatEvent`] to ArcDPS' event processing.
///
/// `is_statechange` will be set to extension, pad61-64 will be set to `sig`.
/// Event will end up processed like ArcDPS events and logged to EVTC.
#[inline]
pub unsafe fn e9_add_event(event: *const RawCombatEvent, sig: u32) {
    (ARC_GLOBALS.e9.expect("failed to find arc export e9"))(event, sig)
}
