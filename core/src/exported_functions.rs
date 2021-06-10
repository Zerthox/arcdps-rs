use crate::raw_structs::*;
use std::{
    ffi::{c_void, CString},
    mem::transmute,
};

static mut ARCDPS: HANDLE = 0 as _;

#[inline(always)]
pub(crate) unsafe fn __set_handle(arcdps: HANDLE) {
    ARCDPS = arcdps;
}

unsafe fn get_func(e: &str) -> *mut c_void {
    let e = CString::new(e).unwrap();
    GetProcAddress(ARCDPS, e.as_ptr())
}

static mut E0: Option<Export0> = None;
pub unsafe fn e0() -> *mut u16 {
    E0.get_or_insert_with(|| transmute(get_func("e0")))()
}

static mut E3: Option<Export3> = None;
pub unsafe fn e3(s: *mut u8) {
    E3.get_or_insert_with(|| transmute(get_func("e3")))(s)
}

static mut E5: Option<Export5> = None;
pub unsafe fn e5(out: *mut [*mut [[f32; 4]]; 5]) {
    E5.get_or_insert_with(|| transmute(get_func("e5")))(out)
}

static mut E6: Option<Export6> = None;
pub unsafe fn e6() -> u64 {
    E6.get_or_insert_with(|| transmute(get_func("e6")))()
}

static mut E7: Option<Export7> = None;
pub unsafe fn e7() -> u64 {
    E7.get_or_insert_with(|| transmute(get_func("e7")))()
}

static mut E8: Option<Export8> = None;
pub unsafe fn e8(s: *mut u8) {
    E8.get_or_insert_with(|| transmute(get_func("e8")))(s)
}
