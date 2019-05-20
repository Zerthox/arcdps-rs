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
    pub fn new(
        sig: usize,
        name: &'static str,
        build: &'static str,
        wnd_nofilter: Option<WndprocCallback>,
        combat: Option<CombatCallback>,
        imgui: Option<ImguiCallback>,
        options_end: Option<OptionsCallback>,
        combat_local: Option<CombatCallback>,
        wnd_filter: Option<WndprocCallback>,
        options_windows: Option<OptionsWindowsCallback>,
    ) -> arcdps_exports {
        arcdps_exports {
            size: std::mem::size_of::<arcdps_exports>(),
            sig,
            out_name: CString::new(name).unwrap().as_ptr() as PCCHAR,
            out_build: CString::new(build).unwrap().as_ptr() as PCCHAR,
            wnd_nofilter: unpack!(wnd_nofilter),
            combat: unpack!(combat),
            imgui: unpack!(imgui),
            options_end: unpack!(options_end),
            combat_local: unpack!(combat_local),
            wnd_filter: unpack!(wnd_filter),
            options_windows: unpack!(options_windows),
        }
    }
}

#[macro_export]
macro_rules! unpack {
    ( $x:expr ) => {{
        match $x {
            Some(func) => func as LPVOID,
            None => null::<isize>() as LPVOID,
        }
    }};
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
