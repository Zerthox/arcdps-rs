use std::{
    ffi::{CStr, c_char},
    slice,
};

/// Helper to convert a string pointer to a [`prim@str`].
#[inline]
pub unsafe fn str_from_cstr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(ptr) }.to_str().ok()
    }
}

/// Helper to convert a string pointer and a length to a [`prim@str`].
///
/// The pointer needs to be non-null. Panics if the string is invalid UTF-8.
#[inline]
#[allow(dead_code)]
pub unsafe fn str_from_cstr_len<'a>(ptr: *const c_char, len: u64) -> &'a str {
    let slice = unsafe { slice::from_raw_parts(ptr as *const u8, len as usize) };
    str::from_utf8(slice).expect("cstr with invalid utf8")
}

/// Strips the `':'` prefix from an account name if present.
#[inline]
pub fn strip_account_prefix(account_name: &str) -> &str {
    account_name.strip_prefix(':').unwrap_or(account_name)
}

/// Helper to define function types with optional unwind ABI.
macro_rules! abi {
    ( $( $vis:vis type $name:ident = unsafe extern fn( $( $args:tt )* ) $( -> $ret:ty )? ; )* ) => {
        $(
            #[cfg(feature = "unwind")]
            $vis type $name = unsafe extern "C-unwind" fn( $( $args )* ) $( -> $ret )?;

            #[cfg(not(feature = "unwind"))]
            $vis type $name = unsafe extern "C" fn( $( $args )* ) $( -> $ret )?;
        )*
    };
}

pub(crate) use abi;
