//! Global instance with Unofficial Extras information.

use crate::{
    extras::exports::raw::{ExportGetKey, ExportGetKeybind},
    util::exported_proc,
};
use std::{mem::transmute, sync::OnceLock};
use windows::Win32::Foundation::HMODULE;

/// Global instance of Unofficial Extras handle & exported functions.
pub static EXTRAS_GLOBALS: OnceLock<ExtrasGlobals> = OnceLock::new();

/// Unofficial Extras handle & exported functions.
pub struct ExtrasGlobals {
    /// Handle to Unofficial Extras dll.
    pub handle: HMODULE,

    /// Unofficial Extras version as string.
    pub version: Option<&'static str>,

    /// Get key export.
    pub get_key: Option<ExportGetKey>,

    /// Get key bind export.
    pub get_keybind: Option<ExportGetKeybind>,
}

impl ExtrasGlobals {
    /// Creates new Unofficial Extras globals.
    pub unsafe fn new(handle: HMODULE, version: Option<&'static str>) -> Self {
        #![allow(clippy::missing_transmute_annotations)]
        Self {
            handle,
            version,
            get_key: transmute(exported_proc(handle, "get_key\0")),
            get_keybind: transmute(exported_proc(handle, "get_key_bind\0")),
        }
    }

    /// Initializes the Unofficial Extras globals.
    pub unsafe fn init(handle: HMODULE, version: Option<&'static str>) -> &'static Self {
        EXTRAS_GLOBALS.get_or_init(|| Self::new(handle, version))
    }

    /// Returns the Unofficial Extras globals.
    #[inline]
    pub fn get() -> &'static Self {
        Self::try_get().expect("unofficial extras globals not initialized")
    }

    /// Tries to retrieve the Unofficial Extras globals.
    #[inline]
    pub fn try_get() -> Option<&'static Self> {
        EXTRAS_GLOBALS.get()
    }
}

unsafe impl Send for ExtrasGlobals {}

unsafe impl Sync for ExtrasGlobals {}
