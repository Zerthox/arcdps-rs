//! Miscellaneous utilities.

use std::{ffi::CStr, os::raw::c_char};

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
