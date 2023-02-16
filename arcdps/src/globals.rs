//! Global instance with ArcDPS information.
#[allow(deprecated)]
use crate::{
    exports::raw::{
        Export0, Export3, Export5, Export6, Export7, Export8, Export9, ExportAddExtension,
        ExportAddExtensionOld, ExportFreeExtension, ExportFreeExtensionOld,
    },
    imgui,
    util::exported_proc,
};
use log::error;
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

    /// [`imgui::Ui`] kept in memory between renders.
    pub ui: Option<imgui::Ui<'static>>,

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

    /// Old add extension export.
    #[allow(deprecated)]
    pub add_extension_old: Option<ExportAddExtensionOld>,

    /// Old free extension export.
    #[allow(deprecated)]
    pub free_extension_old: Option<ExportFreeExtensionOld>,

    /// Add extension export.
    pub add_extension: Option<ExportAddExtension>,

    /// Free extension export.
    pub free_extension: Option<ExportFreeExtension>,
}

impl ArcGlobals {
    /// Creates an empty version of ArcDPS globals.
    const fn empty() -> Self {
        Self {
            handle: HINSTANCE(0),
            version: None,
            ui: None,
            e0: None,
            e3: None,
            e5: None,
            e6: None,
            e7: None,
            e8: None,
            e9: None,
            add_extension_old: None,
            free_extension_old: None,
            add_extension: None,
            free_extension: None,
        }
    }

    /// Initializes the ArcDPS globals.
    pub unsafe fn init(&mut self, handle: HINSTANCE, version: Option<&'static str>) {
        *self = Self {
            handle,
            version,
            ui: IG_CONTEXT.as_ref().map(imgui::Ui::from_ctx),
            e0: transmute(exported_proc(handle, "e0\0")),
            e3: transmute(exported_proc(handle, "e3\0")),
            e5: transmute(exported_proc(handle, "e5\0")),
            e6: transmute(exported_proc(handle, "e6\0")),
            e7: transmute(exported_proc(handle, "e7\0")),
            e8: transmute(exported_proc(handle, "e8\0")),
            e9: transmute(exported_proc(handle, "e9\0")),
            add_extension_old: transmute(exported_proc(handle, "addextension\0")),
            free_extension_old: transmute(exported_proc(handle, "freeextension\0")),
            add_extension: transmute(exported_proc(handle, "addextension2\0")),
            free_extension: transmute(exported_proc(handle, "freeextension2\0")),
        };
    }
}

pub type MallocFn = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;
pub type FreeFn = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);

/// ImGui context.
pub static mut IG_CONTEXT: Option<imgui::Context> = None;

/// Helper to initialize ImGui.
pub unsafe fn init_imgui(
    ctx: *mut imgui::sys::ImGuiContext,
    malloc: Option<MallocFn>,
    free: Option<FreeFn>,
) {
    imgui::sys::igSetCurrentContext(ctx);
    imgui::sys::igSetAllocatorFunctions(malloc, free, ptr::null_mut());
    IG_CONTEXT = Some(imgui::Context::current());
}

/// Available DirectX 11 device.
pub static mut D3D11_DEVICE: Option<ID3D11Device> = None;

/// Helper to initialize DirectX device(s).
pub unsafe fn init_dxgi(id3d: *mut c_void, d3d_version: u32) {
    if !id3d.is_null() && d3d_version == 11 {
        let swap_chain: IDXGISwapChain = unsafe { transmute(id3d) };
        match swap_chain.GetDevice() {
            Ok(device) => D3D11_DEVICE = Some(device),
            Err(err) => error!(target: "both", "failed to get d3d11 device: {err}"),
        }
    }
}
