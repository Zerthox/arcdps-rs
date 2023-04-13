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
    util::exported_proc,
};
use std::{ffi::c_void, mem::transmute, ptr};
use windows::Win32::{
    Foundation::HINSTANCE,
    Graphics::{Direct3D11::ID3D11Device, Dxgi::IDXGISwapChain},
};

/// Global instance of ArcDPS handle & exported functions.
pub static mut ARC_GLOBALS: ArcGlobals = ArcGlobals::empty();

/// ArcDPS handle & exported functions.
// TODO: should we move other globals from codegen here? or move this to codegen?
#[derive(Debug)]
pub struct ArcGlobals {
    /// Handle to ArcDPS dll.
    pub handle: HINSTANCE,

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
    /// Creates an empty version of ArcDPS globals.
    const fn empty() -> Self {
        Self {
            handle: HINSTANCE(0),
            version: None,
            e0: None,
            e3: None,
            e5: None,
            e6: None,
            e7: None,
            e8: None,
            e9: None,
            e10: None,
            add_extension: None,
            free_extension: None,
            list_extension: None,
        }
    }

    /// Initializes the ArcDPS globals.
    pub unsafe fn init(&mut self, handle: HINSTANCE, version: Option<&'static str>) {
        *self = Self {
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
        };
    }
}

pub type MallocFn = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;
pub type FreeFn = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);

/// ImGui context.
pub static mut IG_CONTEXT: Option<imgui::Context> = None;

/// [`imgui::Ui`] kept in memory between renders.
pub static mut IG_UI: Option<imgui::Ui<'static>> = None;

/// Helper to initialize ImGui.
pub unsafe fn init_imgui(
    ctx: *mut imgui::sys::ImGuiContext,
    malloc: Option<MallocFn>,
    free: Option<FreeFn>,
) {
    imgui::sys::igSetCurrentContext(ctx);
    imgui::sys::igSetAllocatorFunctions(malloc, free, ptr::null_mut());
    IG_CONTEXT = Some(imgui::Context::current());
    IG_UI = Some(imgui::Ui::from_ctx(IG_CONTEXT.as_ref().unwrap_unchecked()));
}

/// Current DirectX version.
pub static mut D3D_VERSION: u32 = 0;

/// Returns the current DirectX version.
///
/// `11` for DirectX 11 and `9` for legacy DirectX 9 mode.
#[inline]
pub fn d3d_version() -> u32 {
    unsafe { D3D_VERSION }
}

/// DirectX 11 swap chain.
pub static mut DXGI_SWAP_CHAIN: Option<IDXGISwapChain> = None;

/// Returns the DirectX swap chain, if available.
#[inline]
pub fn dxgi_swap_chain() -> Option<&'static IDXGISwapChain> {
    unsafe { DXGI_SWAP_CHAIN.as_ref() }
}

/// Available DirectX 11 device.
pub static mut D3D11_DEVICE: Option<ID3D11Device> = None;

/// Returns the DirectX 11 device, if available.
#[inline]
pub fn d3d11_device() -> Option<&'static ID3D11Device> {
    unsafe { D3D11_DEVICE.as_ref() }
}

/// Helper to initialize DirectX information.
pub unsafe fn init_dxgi(id3d: *mut c_void, d3d_version: u32, name: &'static str) {
    D3D_VERSION = d3d_version;
    if !id3d.is_null() && d3d_version == 11 {
        // referencing here prevents a crash due to drop
        let swap_chain: &IDXGISwapChain = transmute(&id3d);
        DXGI_SWAP_CHAIN = Some(swap_chain.clone());

        match swap_chain.GetDevice() {
            Ok(device) => D3D11_DEVICE = Some(device),
            Err(err) => {
                let msg = &format!("{name} error: failed to get d3d11 device: {err}");
                if has_e3_log_file() {
                    let _ = log_to_file(msg);
                }
                if has_e8_log_window() {
                    let _ = log_to_window(msg);
                }
            }
        }
    }
}
