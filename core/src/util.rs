//! Miscellaneous utilities.

use crate::extras::{RawUserInfo, UserInfo};
use std::{ffi::CStr, os::raw::c_char};

// TODO: can we move any of this to somewhere better?

/// Reexports for usage in macros.
#[doc(hidden)]
pub mod __macro {
    pub use std::os::raw::{c_char, c_void};
    pub use windows::Win32::{
        Foundation::{HINSTANCE, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP},
    };
}

/// Helper to convert ArcDPS strings to [`str`].
#[inline(always)]
pub fn str_from_cstr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(ptr) }.to_str().ok()
    }
}

/// Helper to convert raw arguments to safe abstractions
#[inline(always)]
pub fn convert_extras_user(user: &RawUserInfo) -> UserInfo {
    let name = str_from_cstr(user.account_name as _);
    UserInfo {
        account_name: name.map(|n| n.trim_start_matches(':')),
        join_time: user.join_time,
        role: user.role,
        subgroup: user.subgroup,
        ready_status: user.ready_status,
    }
}
