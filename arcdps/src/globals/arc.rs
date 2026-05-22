use crate::{
    exports::raw::{
        Export0, Export3, Export5, Export6, Export7, Export8, Export9, Export10,
        ExportAddExtension, ExportFreeExtension, ExportListExtension,
    },
    util::{exported_proc, str_from_cstr},
};
use std::{ffi::c_char, mem::transmute, sync::OnceLock};
use windows::Win32::{Foundation::HMODULE, System::LibraryLoader::GetProcAddress};

/// Initializes ArcDPS information.
#[inline]
pub unsafe fn init_arc(arc_handle: HMODULE, version: *const c_char) {
    unsafe { ArcGlobals::init(arc_handle, str_from_cstr(version)) };
}

/// Global instance of ArcDPS handle & exported functions.
static ARC_GLOBALS: OnceLock<ArcGlobals> = OnceLock::new();

/// ArcDPS handle & exported functions.
// TODO: should we move other globals from codegen here? or move this to codegen?
#[derive(Debug)]
pub struct ArcGlobals {
    /// Handle to ArcDPS dll.
    pub handle: HMODULE,

    /// ArcDPS version as string.
    pub version: Option<&'static str>,

    /// Config path export.
    pub e0: Option<Export0>,

    /// Log file export.
    pub e3: Option<Export3>,

    /// Colors export.
    pub e5: Option<Export5>,

    /// Ui settings export.
    pub e6: Option<Export6>,

    /// Modifiers export.
    pub e7: Option<Export7>,

    /// Log window export.
    pub e8: Option<Export8>,

    /// Add event export.
    pub e9: Option<Export9>,

    /// Add event combat/skill export.
    pub e10: Option<Export10>,

    /// Add extension export.
    pub add_extension: Option<ExportAddExtension>,

    /// Free extension export.
    pub free_extension: Option<ExportFreeExtension>,

    /// List extension export.
    pub list_extension: Option<ExportListExtension>,
}

impl ArcGlobals {
    /// Creates new ArcDPS globals.
    pub unsafe fn new(handle: HMODULE, version: Option<&'static str>) -> Self {
        #![allow(clippy::missing_transmute_annotations)]
        unsafe {
            Self {
                handle,
                version,
                e0: transmute(GetProcAddress(handle, s!("e0"))),
                e3: transmute(GetProcAddress(handle, s!("e3"))),
                e5: transmute(GetProcAddress(handle, s!("e5"))),
                e6: transmute(GetProcAddress(handle, s!("e6"))),
                e7: transmute(GetProcAddress(handle, s!("e7"))),
                e8: transmute(GetProcAddress(handle, s!("e8"))),
                e9: transmute(GetProcAddress(handle, s!("e9"))),
                e10: transmute(GetProcAddress(handle, s!("e10"))),
                add_extension: transmute(GetProcAddress(handle, s!("addextension2"))),
                free_extension: transmute(GetProcAddress(handle, s!("freeextension2"))),
                list_extension: transmute(GetProcAddress(handle, s!("listextension"))),
            }
        }
    }

    /// Initializes the ArcDPS globals.
    pub unsafe fn init(handle: HMODULE, version: Option<&'static str>) -> &'static Self {
        ARC_GLOBALS.get_or_init(|| unsafe { Self::new(handle, version) })
    }

    /// Returns the ArcDPS globals.
    #[inline]
    pub fn get() -> &'static Self {
        Self::try_get().expect("arcdps globals not initialized")
    }

    /// Tries to retrieve the ArcDPS globals.
    #[inline]
    pub fn try_get() -> Option<&'static Self> {
        ARC_GLOBALS.get()
    }
}

unsafe impl Send for ArcGlobals {}

unsafe impl Sync for ArcGlobals {}
