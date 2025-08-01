//! Raw ArcDPS exports.

use crate::{evtc::Event, globals::arc::ArcGlobals, imgui::sys::ImVec4};
use std::ffi::{c_char, c_void};
use windows::Win32::Foundation::HMODULE;

/// Returns the handle to the ArcDPS dll.
pub unsafe fn handle() -> HMODULE {
    ArcGlobals::get().handle
}

/// Signature of the `e0` export. See [`e0_config_path`] for details.
pub type Export0 = unsafe extern "C" fn() -> *const u16;

/// Retrieves path to ArcDPS ini config file as wide char string.
#[inline]
pub unsafe fn e0_config_path() -> *const u16 {
    ArcGlobals::get().e0.expect("failed to find arc export e0")()
}

/// Signature of the `e3` export. See [`e3_log_file`] for details.
pub type Export3 = unsafe extern "C" fn(string: *const c_char);

/// Logs a string to `arcdps.log` file.
#[inline]
pub unsafe fn e3_log_file(string: *const c_char) {
    ArcGlobals::get().e3.expect("failed to find arc export e3")(string)
}

/// Signature of the `e5` export. See [`e5_colors`] for details.
pub type Export5 = unsafe extern "C" fn(out: *mut [*mut ImVec4; 5]);

/// Writes color array pointers to buffer.
#[inline]
pub unsafe fn e5_colors(buffer: *mut [*mut ImVec4; 5]) {
    ArcGlobals::get().e5.expect("failed to find arc export e5")(buffer)
}

/// Signature of the `e6` export. See [`e6_ui_settings`] for details.
pub type Export6 = unsafe extern "C" fn() -> u64;

/// Retrieves bit mask of current ArcDPS UI settings.
#[inline]
pub unsafe fn e6_ui_settings() -> u64 {
    ArcGlobals::get().e6.expect("failed to find arc export e6")()
}

/// Signature of the `e7` export. See [`e7_modifiers`] for details.
pub type Export7 = unsafe extern "C" fn() -> u64;

/// Retrieves modifier keys as virtual key codes.
#[inline]
pub unsafe fn e7_modifiers() -> u64 {
    ArcGlobals::get().e7.expect("failed to find arc export e7")()
}

/// Signature of the `e8` export. See [`e8_log_window`] for details.
pub type Export8 = unsafe extern "C" fn(string: *const c_char);

/// Logs a string to the ArcDPS logger window.
///
/// Colors are HTML-like: `<c=#aaaaaa>colored text</c>`.
#[inline]
pub unsafe fn e8_log_window(string: *const c_char) {
    ArcGlobals::get().e8.expect("failed to find arc export e8")(string)
}

/// Signature of the `e9` export. See [`e9_add_event`] for details.
pub type Export9 = unsafe extern "C" fn(event: *const Event, sig: u32);

/// Adds an [`Event`] to ArcDPS' event processing.
///
/// `is_statechange` will be set to [`StateChange::Extension`](crate::evtc::StateChange::Extension), pad61-64 will be set to `sig`.
/// Event will end up processed like ArcDPS events and logged to EVTC.
#[inline]
pub unsafe fn e9_add_event(event: *const Event, sig: u32) {
    ArcGlobals::get().e9.expect("failed to find arc export e9")(event, sig)
}

/// Signature of the `e10` export. See [`e10_add_event_combat`] for details.
pub type Export10 = unsafe extern "C" fn(event: *const Event, sig: u32);

/// Adds an [`Event`] to ArcDPS' event processing.
///
/// `is_statechange` will be set to [`StateChange::ExtensionCombat`](crate::evtc::StateChange::ExtensionCombat), pad61-64 will be set to `sig`.
/// Event will end up processed like ArcDPS events and logged to EVTC.
///
/// Contrary to [`e9_add_event`], the `skill_id` is treated as skill id and will be added to the EVTC skill table.
#[inline]
pub unsafe fn e10_add_event_combat(event: *const Event, sig: u32) {
    ArcGlobals::get()
        .e10
        .expect("failed to find arc export e10")(event, sig)
}

/// Signature of the `addextension2` export. See [`add_extension`] for details.
pub type ExportAddExtension = unsafe extern "C" fn(handle: HMODULE) -> u32;

/// Requests to load an extension (plugin/addon).
///
/// ArcDPS will `LoadLibrary` the `handle` to increment the reference count, call `get_init_addr` and call its returned function.
/// Returns `0` on success or non-zero on error. See [`AddExtensionResult`](super::AddExtensionResult) for details.
///
/// This uses version 2 (`addextension2`) of the extension API.
#[inline]
pub unsafe fn add_extension(handle: HMODULE) -> u32 {
    ArcGlobals::get()
        .add_extension
        .expect("failed to find arc export addextension2")(handle)
}

/// Signature of the `freeextension2` export. See [`free_extension`] for details.
pub type ExportFreeExtension = unsafe extern "C" fn(sig: u32) -> HMODULE;

/// Requests to free a loaded extension (plugin/addon).
///
/// ArcDPS will call `get_release_addr` and its returned function.
/// Upon returning from [`free_extension`] there will be no more pending callbacks.
/// However, the caller must ensure no callbacks are executing before freeing.
/// Returns `0` if extension was not found or [`HMODULE`] handle of the module otherwise.
///
/// This uses version 2 (`freeextension2`) of the extension API.
#[inline]
pub unsafe fn free_extension(sig: u32) -> HMODULE {
    ArcGlobals::get()
        .free_extension
        .expect("failed to find arc export freeextension2")(sig)
}

/// Signature of the `listextension` export. See [`list_extension`] for details.
pub type ExportListExtension = unsafe extern "C" fn(callback_fn: *const c_void);

/// Retrieves a list of extensions via callback.
///
/// `callback_fn` is a callback of type `void callback_fn(arcdps_exports* exp)`.
/// Callback is called once for each extension current loaded.
#[inline]
pub unsafe fn list_extension(callback_fn: *const c_void) {
    // TODO: is this sync?
    // TODO: bindings should check for uninitialized
    ArcGlobals::get()
        .list_extension
        .expect("failed to find arc export listextension")(callback_fn)
}
