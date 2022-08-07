//! Miscellaneous utilities.

use std::{ffi::CStr, os::raw::c_char, slice, str};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{FARPROC, HINSTANCE},
        System::LibraryLoader::GetProcAddress,
    },
};

/// Helper to convert a string pointer to a [`str`].
#[inline]
pub unsafe fn str_from_cstr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        None
    } else {
        CStr::from_ptr(ptr).to_str().ok()
    }
}

/// Helper to convert a string pointer and a length to a [`str`].
///
/// The pointer needs to be non-null. Panics if the string is invalid UTF-8.
#[inline]
pub unsafe fn str_from_cstr_len<'a>(ptr: *const c_char, len: u64) -> &'a str {
    let slice = slice::from_raw_parts(ptr as *const u8, len as usize);
    str::from_utf8(slice).expect("cstr with invalid utf8")
}

/// Helper to retrieve an exported function.
/// Name needs to be null-terminated.
#[inline]
pub unsafe fn exported_proc(handle: HINSTANCE, name: &'static str) -> FARPROC {
    GetProcAddress(handle, PCSTR(name.as_ptr()))
}
