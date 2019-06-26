use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr::null,
};
use winapi::shared::{
    minwindef::{LPARAM, LPVOID, UINT, WPARAM},
    ntdef::PCCHAR,
    windef::HWND,
};

static mut FUNCTIONS: Option<ArcdpsFunctions> = None;
static mut INFO: Option<(CString, CString)> = None;

pub type WndprocCallback = fn(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> usize;
/*
type CombatCallback = fn(
    ev: *mut cbtevent,
    src: *mut u_ag,
    dst: *mut u_ag,
    skillname: PCCHAR,
    id: u64,
    revision: u64,
) -> usize;
type ImguiCallback = fn(not_charsel_or_loading: u32) -> usize;
type OptionsCallback = fn() -> usize;
type OptionsWindowsCallback = fn(windowname: PCCHAR) -> usize;
*/

// pub type SafeWndprocCallback = ?
pub type SafeCombatCallback = fn(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&str>,
    id: u64,
    revision: u64,
);
pub type SafeImguiCallback = fn(not_charsel_or_loading: bool);
pub type SafeOptionsCallback = fn();
pub type SafeOptionsWindowsCallback = fn(windowname: Option<&str>);

struct ArcdpsFunctions {
    // pub wnd_nofilter: SafeWndprocCallback,
    pub combat: Option<SafeCombatCallback>,
    pub imgui: Option<SafeImguiCallback>,
    pub options_end: Option<SafeOptionsCallback>,
    pub combat_local: Option<SafeCombatCallback>,
    // pub wnd_filter: SafeWndprocCallback,
    pub options_windows: Option<SafeOptionsWindowsCallback>,
}

fn options_wrapper() -> usize {
    let func = unsafe {
        if let Some(funcs) = &FUNCTIONS {
            funcs.options_end.unwrap()
        } else {
            return 0;
        }
    };
    func();
    0
}

fn options_windows_wrapper(windowname: PCCHAR) -> usize {
    let func = unsafe {
        if let Some(funcs) = &FUNCTIONS {
            funcs.options_windows.unwrap()
        } else {
            return 0;
        }
    };
    func(get_str_from_pcchar(windowname));
    0
}

fn cbt_wrapper_area(
    ev: *mut cbtevent,
    src: *mut u_ag,
    dst: *mut u_ag,
    skillname: PCCHAR,
    id: u64,
    revision: u64,
) -> usize {
    let func = unsafe {
        if let Some(funcs) = &FUNCTIONS {
            funcs.combat.unwrap()
        } else {
            return 0;
        }
    };
    cbt_wrapper(func, ev, src, dst, skillname, id, revision)
}

fn cbt_wrapper_local(
    ev: *mut cbtevent,
    src: *mut u_ag,
    dst: *mut u_ag,
    skillname: PCCHAR,
    id: u64,
    revision: u64,
) -> usize {
    let func = unsafe {
        if let Some(funcs) = &FUNCTIONS {
            funcs.combat_local.unwrap()
        } else {
            return 0;
        }
    };
    cbt_wrapper(func, ev, src, dst, skillname, id, revision)
}

fn cbt_wrapper(
    func: SafeCombatCallback,
    ev: *mut cbtevent,
    src: *mut u_ag,
    dst: *mut u_ag,
    skillname: PCCHAR,
    id: u64,
    revision: u64,
) -> usize {
    let s_ev: Option<&cbtevent>;
    let s_src: Option<&Ag>;
    let s_dst: Option<&Ag>;
    let s_skillname: Option<&str>;
    let p_src;
    let p_dst;
    unsafe {
        s_ev = if ev.is_null() { None } else { Some(&*ev) };
        s_src = if src.is_null() {
            None
        } else {
            p_src = get_safe_ag(&*src);
            Some(&p_src)
        };
        s_dst = if dst.is_null() {
            None
        } else {
            p_dst = get_safe_ag(&*dst);
            Some(&p_dst)
        };
        s_skillname = get_str_from_pcchar(skillname);
        func
    };
    func(s_ev, s_src, s_dst, s_skillname, id, revision);
    0
}

fn imgui_wrapper(not_charsel_or_loading: u32) -> usize {
    let func = unsafe {
        if let Some(funcs) = &FUNCTIONS {
            funcs.imgui.unwrap()
        } else {
            return 0;
        }
    };
    func(not_charsel_or_loading != 0);
    0
}

fn get_safe_ag(ag: &u_ag) -> Ag {
    let name = get_str_from_pcchar(ag.name);
    Ag {
        name,
        id: ag.id,
        prof: ag.prof,
        elite: ag.elite,
        self_: ag.self_,
        team: ag.team,
    }
}

fn get_str_from_pcchar<'a>(src: PCCHAR) -> Option<&'a str> {
    if src.is_null() {
        None
    } else {
        Some(unsafe {
            CStr::from_ptr(src as *const c_char)
                .to_str()
                .unwrap_or_default()
        })
    }
}

impl arcdps_exports {
    pub fn new(sig: usize, name: &'static str, build: &'static str) -> arcdps_exports {
        let (name, build) = unsafe {
            FUNCTIONS = Some(ArcdpsFunctions {
                combat: None,
                imgui: None,
                options_end: None,
                combat_local: None,
                options_windows: None,
            });
            INFO = Some((CString::new(name).unwrap(), CString::new(build).unwrap()));
            if let Some(infos) = &INFO {
                infos
            } else {
                unreachable!();
            }
        };
        arcdps_exports {
            size: std::mem::size_of::<arcdps_exports>(),
            sig,
            out_name: name.as_ptr() as PCCHAR,
            out_build: build.as_ptr() as PCCHAR,
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

    pub fn combat(mut self, func: SafeCombatCallback) -> Self {
        self.combat = cbt_wrapper_area as LPVOID;
        unsafe {
            if let Some(funcs) = &mut FUNCTIONS {
                funcs.combat = Some(func);
            };
        }
        self
    }

    pub fn imgui(mut self, func: SafeImguiCallback) -> Self {
        self.imgui = imgui_wrapper as LPVOID;
        unsafe {
            if let Some(funcs) = &mut FUNCTIONS {
                funcs.imgui = Some(func);
            };
        }
        self
    }

    pub fn options_end(mut self, func: SafeOptionsCallback) -> Self {
        self.options_end = options_wrapper as LPVOID;
        unsafe {
            if let Some(funcs) = &mut FUNCTIONS {
                funcs.options_end = Some(func);
            };
        }
        self
    }

    pub fn combat_local(mut self, func: SafeCombatCallback) -> Self {
        self.combat_local = cbt_wrapper_local as LPVOID;
        unsafe {
            if let Some(funcs) = &mut FUNCTIONS {
                funcs.combat_local = Some(func);
            };
        }
        self
    }

    pub fn wnd_filter(mut self, func: WndprocCallback) -> Self {
        self.wnd_filter = func as LPVOID;
        self
    }

    pub fn options_windows(mut self, func: SafeOptionsWindowsCallback) -> Self {
        self.options_windows = options_windows_wrapper as LPVOID;
        unsafe {
            if let Some(funcs) = &mut FUNCTIONS {
                funcs.options_windows = Some(func);
            };
        }
        self
    }
}

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
struct u_ag {
    pub name: PCCHAR,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Ag<'a> {
    pub name: Option<&'a str>,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}
