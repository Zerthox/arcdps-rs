//! Global instance with ArcDPS information.

use crate::{
    api::RawCombatEvent,
    imgui::{self, sys::ImVec4, Context, Ui},
    util::exported_proc,
};
use std::{ffi::c_void, mem::transmute, os::raw::c_char, ptr};
use windows::Win32::Foundation::HINSTANCE;

/// Global instance of Arc handle & exported functions.
pub static mut ARC_INSTANCE: ArcInstance = ArcInstance::empty();

type Export0 = unsafe extern "C" fn() -> *const u16;
type Export3 = unsafe extern "C" fn(*mut c_char);
type Export5 = unsafe extern "C" fn(*mut [*mut ImVec4; 5]);
type Export6 = unsafe extern "C" fn() -> u64;
type Export7 = unsafe extern "C" fn() -> u64;
type Export8 = unsafe extern "C" fn(*mut c_char);
type Export9 = unsafe extern "C" fn(*mut RawCombatEvent, u32);

/// Arc handle & exported functions.
// TODO: should we move other globals from codegen here? or move this to codegen?
#[derive(Debug)]
pub struct ArcInstance {
    pub handle: HINSTANCE,
    pub version: Option<&'static str>,
    pub ui: Option<Ui<'static>>,
    pub e0: Option<Export0>,
    pub e3: Option<Export3>,
    pub e5: Option<Export5>,
    pub e6: Option<Export6>,
    pub e7: Option<Export7>,
    pub e8: Option<Export8>,
    pub e9: Option<Export9>,
}

impl ArcInstance {
    pub const fn empty() -> Self {
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
        }
    }

    /// Creates a new Arc handle & exports instance.
    pub unsafe fn new(handle: HINSTANCE, version: Option<&'static str>) -> Self {
        Self {
            handle,
            version,
            ui: IG_CONTEXT.as_ref().map(Ui::from_ctx),
            e0: transmute(exported_proc(handle, "e0\0")),
            e3: transmute(exported_proc(handle, "e3\0")),
            e5: transmute(exported_proc(handle, "e5\0")),
            e6: transmute(exported_proc(handle, "e6\0")),
            e7: transmute(exported_proc(handle, "e7\0")),
            e8: transmute(exported_proc(handle, "e8\0")),
            e9: transmute(exported_proc(handle, "e9\0")),
        }
    }

    /// Initializes the Arc handle & exports.
    pub unsafe fn init(&mut self, handle: HINSTANCE, version: Option<&'static str>) {
        *self = Self::new(handle, version);
    }
}

pub type MallocFn = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;
pub type FreeFn = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);

/// Imgui context.
pub static mut IG_CONTEXT: Option<Context> = None;

/// Helper to initialize Imgui.
pub(crate) unsafe fn init_imgui(
    ctx: *mut imgui::sys::ImGuiContext,
    malloc: Option<MallocFn>,
    free: Option<FreeFn>,
) {
    imgui::sys::igSetCurrentContext(ctx);
    imgui::sys::igSetAllocatorFunctions(malloc, free, ptr::null_mut());
    IG_CONTEXT = Some(Context::current());
}
