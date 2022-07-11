use crate::{api::RawCombatEvent, imgui::sys::ImVec4, instance::ARC_INSTANCE};
use std::os::raw::c_char;

/// Retrieves path to ArcDPS ini config file as wide char string.
pub unsafe fn e0_config_path() -> *const u16 {
    (ARC_INSTANCE.e0.unwrap())()
}

/// Logs a string to `arcdps.log` file.
pub unsafe fn e3_log_file(string: *const c_char) {
    (ARC_INSTANCE.e3.unwrap())(string)
}

/// Retrieves color pointers as array. Writes to buffer.
pub unsafe fn e5_colors(buffer: *mut [*mut ImVec4; 5]) {
    (ARC_INSTANCE.e5.unwrap())(buffer)
}

/// Retrieves bit mask of current ArcDPS UI settings.
pub unsafe fn e6_ui_settings() -> u64 {
    (ARC_INSTANCE.e6.unwrap())()
}

/// Retrieves modifier keys as virtual key codes.
pub unsafe fn e7_modifiers() -> u64 {
    (ARC_INSTANCE.e7.unwrap())()
}

/// Logs a string to the ArcDPS logger window.
///
/// Colors are HTML-like: `<c=#aaaaaa>colored text</c>`.
pub unsafe fn e8_log_window(string: *const c_char) {
    (ARC_INSTANCE.e8.unwrap())(string)
}

/// Adds a [`RawCombatEvent`] to ArcDPS' event processing.
///
/// `is_statechange` will be set to extension, pad61-64 will be set to `sig`.
/// Event will end up processed like ArcDPS events and logged to EVTC.
pub unsafe fn e9_add_event(event: *const RawCombatEvent, sig: u32) {
    (ARC_INSTANCE.e9.unwrap())(event, sig)
}
