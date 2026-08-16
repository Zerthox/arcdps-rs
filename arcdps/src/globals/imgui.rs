use crate::{
    imgui::{self, Context, Ui},
    util::Share,
};
use std::{ffi::c_void, mem::ManuallyDrop, ptr, sync::OnceLock};

/// Dear ImGui version the bindings were compiled against.
///
/// arcdps compares this against its own `IMGUI_VERSION_NUM` and drops the UI callbacks of any
/// plugin that disagrees, so it has to come from the headers rather than be hardcoded.
pub const IMGUI_VERSION: u32 = imgui::sys::IMGUI_VERSION_NUM;

pub type MallocFn = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;

pub type FreeFn = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);

/// ImGui context.
///
/// Owned by arcdps, hence [`ManuallyDrop`]: dropping an [`imgui::Context`] destroys the
/// underlying context.
pub static IG_CONTEXT: OnceLock<Share<ManuallyDrop<Context>>> = OnceLock::new();

/// Initializes ImGui information.
pub unsafe fn init_imgui(
    ctx: *mut imgui::sys::ImGuiContext,
    malloc: Option<MallocFn>,
    free: Option<FreeFn>,
) {
    IG_CONTEXT.get_or_init(|| unsafe {
        imgui::sys::igSetCurrentContext(ctx);
        imgui::sys::igSetAllocatorFunctions(malloc, free, ptr::null_mut());
        Share::new(Context::current_borrowed())
    });
}

/// Retrieves the [`imgui::Context`].
#[inline]
pub unsafe fn imgui_context() -> &'static Context {
    unsafe {
        IG_CONTEXT
            .get()
            .expect("imgui context not initialized")
            .get()
    }
}

/// Retrieves the [`imgui::Ui`] for rendering.
#[inline]
pub unsafe fn with_ui<R>(body: impl FnOnce(&Ui) -> R) -> R {
    body(unsafe { imgui_context() }.borrowed_frame())
}
