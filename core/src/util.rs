//! Miscellaneous utilities.

use std::{ffi::CStr, os::raw::c_char};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{FARPROC, HINSTANCE},
        System::LibraryLoader::GetProcAddress,
    },
};

/// Helper to convert ArcDPS strings to [`str`].
#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn str_from_cstr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(ptr) }.to_str().ok()
    }
}

/// Helper to retrieve an exported function.
/// Name needs to be null-terminated.
#[inline]
pub unsafe fn exported_proc(handle: HINSTANCE, name: &'static str) -> FARPROC {
    GetProcAddress(handle, PCSTR(name.as_ptr()))
}
