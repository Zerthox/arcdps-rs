use std::{ffi::CString, ptr::null};
use winapi::shared::{
    minwindef::{LPARAM, LPVOID, UINT, WPARAM},
    ntdef::PCCHAR,
    windef::HWND,
};

pub type WndprocCallback = fn(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> usize;
pub type CombatCallback = fn(
    ev: *mut cbtevent,
    src: *mut ag,
    dst: *mut ag,
    skillname: PCCHAR,
    id: u64,
    revision: u64,
) -> usize;
pub type ImguiCallback = fn(not_charsel_or_loading: u32) -> usize;
pub type OptionsCallback = fn() -> usize;
pub type OptionsWindowsCallback = fn(windowname: PCCHAR) -> usize;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arcdps_exports {
    pub size: usize,
    pub sig: usize,
    pub out_name: PCCHAR,
    pub out_build: PCCHAR,
    pub wnd_nofilter: LPVOID,
    pub combat: LPVOID,
    pub imgui: LPVOID,
    pub options_end: LPVOID,
    pub combat_local: LPVOID,
    pub wnd_filter: LPVOID,
    pub options_windows: LPVOID,
}

impl arcdps_exports {
    pub fn new(sig: usize, name: &'static str, build: &'static str) -> arcdps_exports {
        arcdps_exports {
            size: std::mem::size_of::<arcdps_exports>(),
            sig,
            out_name: CString::new(name).unwrap().as_ptr() as PCCHAR,
            out_build: CString::new(build).unwrap().as_ptr() as PCCHAR,
            wnd_nofilter: null::<isize>() as LPVOID,
            combat: null::<isize>() as LPVOID,
            imgui: null::<isize>() as LPVOID,
            options_end: null::<isize>() as LPVOID,
            combat_local: null::<isize>() as LPVOID,
            wnd_filter: null::<isize>() as LPVOID,
            options_windows: null::<isize>() as LPVOID,
        }
    }

    pub fn wnd_nofilter(mut self, func: WndprocCallback) -> Self {
        self.wnd_nofilter = func as LPVOID;
        self
    }

    pub fn combat(mut self, func: CombatCallback) -> Self {
        self.combat = func as LPVOID;
        self
    }

    pub fn imgui(mut self, func: ImguiCallback) -> Self {
        self.imgui = func as LPVOID;
        self
    }

    pub fn options_end(mut self, func: OptionsCallback) -> Self {
        self.options_end = func as LPVOID;
        self
    }

    pub fn combat_local(mut self, func: CombatCallback) -> Self {
        self.combat_local = func as LPVOID;
        self
    }

    pub fn wnd_filter(mut self, func: WndprocCallback) -> Self {
        self.wnd_filter = func as LPVOID;
        self
    }

    pub fn options_windows(mut self, func: OptionsWindowsCallback) -> Self {
        self.options_windows = func as LPVOID;
        self
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cbtevent {
    pub time: u64,
    pub src_agent: usize,
    pub dst_agent: usize,
    pub value: i32,
    pub buff_dmg: i32,
    pub overstack_value: u32,
    pub skillid: u32,
    pub src_instid: u16,
    pub dst_instid: u16,
    pub src_master_instid: u16,
    pub dst_master_instid: u16,
    pub iff: u8,
    pub buff: u8,
    pub result: u8,
    pub is_activation: u8,
    pub is_buffremove: u8,
    pub is_ninety: u8,
    pub is_fifty: u8,
    pub is_moving: u8,
    pub is_statechange: u8,
    pub is_flanking: u8,
    pub is_shields: u8,
    pub is_offcycle: u8,
    pub pad61: u8,
    pub pad62: u8,
    pub pad63: u8,
    pub pad64: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ag {
    pub name: PCCHAR,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}
