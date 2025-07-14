use crate::{
    imgui::{self, Context, Ui},
    util::Share,
};
use std::{ffi::c_void, ptr, sync::OnceLock};

pub type MallocFn = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;

pub type FreeFn = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);

/// ImGui context.
pub static IG_CONTEXT: OnceLock<Share<Context>> = OnceLock::new();

thread_local! {
    /// ImGui UI.
    pub static IG_UI: Ui<'static> = Ui::from_ctx(unsafe { imgui_context() });
}

/// Initializes ImGui information.
pub unsafe fn init_imgui(
    ctx: *mut imgui::sys::ImGuiContext,
    malloc: Option<MallocFn>,
    free: Option<FreeFn>,
) {
    imgui::sys::igSetCurrentContext(ctx);
    imgui::sys::igSetAllocatorFunctions(malloc, free, ptr::null_mut());
    IG_CONTEXT.get_or_init(|| Share::new(Context::current()));
}

/// Retrieves the [`imgui::Context`].
#[inline]
pub unsafe fn imgui_context() -> &'static Context {
    IG_CONTEXT
        .get()
        .expect("imgui context not initialized")
        .get()
}

/// Retrieves the [`imgui::Ui`] for rendering.
#[inline]
pub unsafe fn with_ui<R>(body: impl FnOnce(&Ui<'static>) -> R) -> R {
    IG_UI.with(body)
}
