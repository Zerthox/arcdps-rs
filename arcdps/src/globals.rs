//! Global instance with ArcDPS information.

use crate::{
    exports::{
        has_e3_log_file, has_e8_log_window, log_to_file, log_to_window,
        raw::{
            Export0, Export10, Export3, Export5, Export6, Export7, Export8, Export9,
            ExportAddExtension, ExportFreeExtension, ExportListExtension,
        },
    },
    imgui,
    util::{exported_proc, Share},
};
use std::{
    ffi::c_void,
    mem::transmute,
    ptr::{self, NonNull},
    sync::{
        atomic::{AtomicU32, Ordering},
        OnceLock,
    },
};
use windows::{
    core::Interface,
    Win32::{
        Foundation::HMODULE,
        Graphics::{Direct3D11::ID3D11Device, Dxgi::IDXGISwapChain},
    },
};

/// Global instance of ArcDPS handle & exported functions.
pub static ARC_GLOBALS: OnceLock<ArcGlobals> = OnceLock::new();

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
        Self {
            handle,
            version,
            e0: transmute(exported_proc(handle, "e0\0")),
            e3: transmute(exported_proc(handle, "e3\0")),
            e5: transmute(exported_proc(handle, "e5\0")),
            e6: transmute(exported_proc(handle, "e6\0")),
            e7: transmute(exported_proc(handle, "e7\0")),
            e8: transmute(exported_proc(handle, "e8\0")),
            e9: transmute(exported_proc(handle, "e9\0")),
            e10: transmute(exported_proc(handle, "e10\0")),
            add_extension: transmute(exported_proc(handle, "addextension2\0")),
            free_extension: transmute(exported_proc(handle, "freeextension2\0")),
            list_extension: transmute(exported_proc(handle, "listextension\0")),
        }
    }

    /// Initializes the ArcDPS globals.
    pub unsafe fn init(handle: HMODULE, version: Option<&'static str>) -> &'static Self {
        ARC_GLOBALS.get_or_init(|| Self::new(handle, version))
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

pub type MallocFn = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;

pub type FreeFn = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);

/// ImGui context.
pub static IG_CONTEXT: OnceLock<Share<imgui::Context>> = OnceLock::new();

/// ImGui UI.
pub static IG_UI: OnceLock<Share<imgui::Ui<'static>>> = OnceLock::new();

/// Helper to initialize ImGui.
pub unsafe fn init_imgui(
    ctx: *mut imgui::sys::ImGuiContext,
    malloc: Option<MallocFn>,
    free: Option<FreeFn>,
) {
    imgui::sys::igSetCurrentContext(ctx);
    imgui::sys::igSetAllocatorFunctions(malloc, free, ptr::null_mut());
    let ctx = IG_CONTEXT.get_or_init(|| Share::new(imgui::Context::current()));
    IG_UI.get_or_init(|| Share::new(imgui::Ui::from_ctx(ctx.get())));
}

/// Current DirectX version.
pub static D3D_VERSION: AtomicU32 = AtomicU32::new(0);

/// Returns the current DirectX version.
///
/// `11` for DirectX 11 and `9` for legacy DirectX 9 mode.
#[inline]
pub fn d3d_version() -> u32 {
    D3D_VERSION.load(Ordering::Relaxed)
}

/// DirectX 11 swap chain.
pub static DXGI_SWAP_CHAIN: OnceLock<Share<NonNull<c_void>>> = OnceLock::new();

/// Returns the DirectX swap chain, if available.
#[inline]
pub fn dxgi_swap_chain() -> Option<IDXGISwapChain> {
    DXGI_SWAP_CHAIN.get().map(|share| {
        unsafe { IDXGISwapChain::from_raw_borrowed(&share.get().as_ptr()) }
            .expect("invalid swap chain")
            .clone()
    })
}

/// Returns the DirectX 11 device, if available.
#[inline]
pub fn d3d11_device() -> Option<ID3D11Device> {
    let swap_chain = dxgi_swap_chain()?;
    unsafe { swap_chain.GetDevice() }.ok()
}

/// Helper to initialize DirectX information.
pub unsafe fn init_dxgi(id3d: *const c_void, d3d_version: u32, name: &'static str) {
    D3D_VERSION.store(d3d_version, Ordering::Relaxed);
    if d3d_version == 11 {
        if let Some(id3d) = NonNull::new(id3d.cast_mut()) {
            let ptr = id3d.as_ptr();
            let swap_chain =
                unsafe { IDXGISwapChain::from_raw_borrowed(&ptr) }.expect("invalid swap chain");

            if let Err(err) = swap_chain.GetDevice::<ID3D11Device>() {
                let msg = &format!("{name} error: failed to get d3d11 device: {err}");
                if has_e3_log_file() {
                    let _ = log_to_file(msg);
                }
                if has_e8_log_window() {
                    let _ = log_to_window(msg);
                }
            }

            DXGI_SWAP_CHAIN.get_or_init(|| Share::new(id3d));
        }
    }
}
