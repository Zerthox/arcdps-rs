//! Global instance with ArcDPS information.

use crate::{api::RawCombatEvent, imgui::sys::ImVec4};
use std::{ffi::CStr, mem::transmute};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{FARPROC, HINSTANCE},
        System::LibraryLoader::GetProcAddress,
    },
};

/// Global instance of Arc handle & exported functions.
pub static mut ARC_INSTANCE: Option<ArcInstance> = None;

/// Arc handle & exported functions.
// TODO: should we move other globals from codegen here? or move this to codegen?
#[derive(Debug)]
pub struct ArcInstance {
    pub handle: HINSTANCE,
    pub version: &'static str,
    pub e0: unsafe extern "C" fn() -> *mut u16,
    pub e3: unsafe extern "C" fn(*mut u8),
    pub e5: unsafe extern "C" fn(*mut [*mut ImVec4; 5]),
    pub e6: unsafe extern "C" fn() -> u64,
    pub e7: unsafe extern "C" fn() -> u64,
    pub e8: unsafe extern "C" fn(*mut u8),
    pub e9: unsafe extern "C" fn(*mut RawCombatEvent, u32),
}

impl ArcInstance {
    /// Initializes the Arc instance with a handle.
    ///
    /// Returns `true` if initialization was successful.
    pub unsafe fn init(handle: HINSTANCE, version: &'static CStr) -> bool {
        ARC_INSTANCE = Self::new(handle, version);
        ARC_INSTANCE.is_some()
    }

    /// Creates a new Arc handle & exports instance.
    unsafe fn new(handle: HINSTANCE, version: &'static CStr) -> Option<Self> {
        Some(Self {
            handle,
            version: version.to_str().ok()?,
            e0: transmute(get_func(handle, "e0\0")?),
            e3: transmute(get_func(handle, "e3\0")?),
            e5: transmute(get_func(handle, "e5\0")?),
            e6: transmute(get_func(handle, "e6\0")?),
            e7: transmute(get_func(handle, "e7\0")?),
            e8: transmute(get_func(handle, "e8\0")?),
            e9: transmute(get_func(handle, "e9\0")?),
        })
    }
}

/// Helper to retrieve an exported function.
/// Name needs to be null-terminated.
unsafe fn get_func(handle: HINSTANCE, name: &'static str) -> FARPROC {
    GetProcAddress(handle, PCSTR(name.as_ptr()))
}
