use once_cell::sync::OnceCell;
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
use core::convert::From;

static FUNCTIONS: OnceCell<ArcdpsFunctions> = OnceCell::new();
static INFO: OnceCell<(CString, CString)> = OnceCell::new();
static EXPORTED: OnceCell<arcdps_exports> = OnceCell::new();

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

impl arcdps_exports {
    pub fn new(sig: usize, name: &'static str, build: &'static str) -> ArcdpsExportsBuilder {
        let (out_name, out_build) = {
            let info =
                INFO.get_or_init(|| (CString::new(name).unwrap(), CString::new(build).unwrap()));
            let (name, build) = &info;
            (name.as_ptr() as PCCHAR, build.as_ptr() as PCCHAR)
        };
        ArcdpsExportsBuilder {
            arcdps: arcdps_exports {
                size: std::mem::size_of::<arcdps_exports>(),
                sig,
                out_name,
                out_build,
                wnd_nofilter: null::<isize>() as LPVOID,
                combat: null::<isize>() as LPVOID,
                imgui: null::<isize>() as LPVOID,
                options_end: null::<isize>() as LPVOID,
                combat_local: null::<isize>() as LPVOID,
                wnd_filter: null::<isize>() as LPVOID,
                options_windows: null::<isize>() as LPVOID,
            },
            funcs: ArcdpsFunctions {
                combat: None,
                imgui: None,
                options_end: None,
                combat_local: None,
                options_windows: None,
            },
        }
    }
}

impl ArcdpsExportsBuilder {
    pub fn wnd_nofilter(mut self, func: WndprocCallback) -> Self {
        self.arcdps.wnd_nofilter = func as LPVOID;
        self
    }

    pub fn combat(mut self, func: SafeCombatCallback) -> Self {
        self.arcdps.combat = cbt_wrapper_area as LPVOID;
        self.funcs.combat = Some(func);
        self
    }

    pub fn imgui(mut self, func: SafeImguiCallback) -> Self {
        self.arcdps.imgui = imgui_wrapper as LPVOID;
        self.funcs.imgui = Some(func);
        self
    }

    pub fn options_end(mut self, func: SafeOptionsCallback) -> Self {
        self.arcdps.options_end = options_wrapper as LPVOID;
        self.funcs.options_end = Some(func);
        self
    }

    pub fn combat_local(mut self, func: SafeCombatCallback) -> Self {
        self.arcdps.combat_local = cbt_wrapper_local as LPVOID;
        self.funcs.combat_local = Some(func);
        self
    }

    pub fn wnd_filter(mut self, func: WndprocCallback) -> Self {
        self.arcdps.wnd_filter = func as LPVOID;
        self
    }

    pub fn options_windows(mut self, func: SafeOptionsWindowsCallback) -> Self {
        self.arcdps.options_windows = options_windows_wrapper as LPVOID;
        self.funcs.options_windows = Some(func);
        self
    }

    pub fn save(self) -> LPVOID {
        let exported = EXPORTED.get_or_init(|| self.arcdps);
        FUNCTIONS.get_or_init(|| self.funcs);
        exported as *const arcdps_exports as LPVOID
    }
}

pub struct ArcdpsExportsBuilder {
    arcdps: arcdps_exports,
    funcs: ArcdpsFunctions,
}

struct ArcdpsFunctions {
    // pub wnd_nofilter: SafeWndprocCallback,
    pub combat: Option<SafeCombatCallback>,
    pub imgui: Option<SafeImguiCallback>,
    pub options_end: Option<SafeOptionsCallback>,
    pub combat_local: Option<SafeCombatCallback>,
    // pub wnd_filter: SafeWndprocCallback,
    pub options_windows: Option<SafeOptionsWindowsCallback>,
}

unsafe fn options_wrapper() -> usize {
    let funcs = FUNCTIONS.get().unwrap();
    let func = funcs.options_end.unwrap();
    func();
    0
}

unsafe fn options_windows_wrapper(windowname: PCCHAR) -> usize {
    let funcs = FUNCTIONS.get().unwrap();
    let func = funcs.options_windows.unwrap();
    func(get_str_from_pcchar(windowname));
    0
}

unsafe fn cbt_wrapper_area(
    ev: *mut cbtevent,
    src: *mut u_ag,
    dst: *mut u_ag,
    skillname: PCCHAR,
    id: u64,
    revision: u64,
) -> usize {
    let funcs = FUNCTIONS.get().unwrap();
    let func = funcs.combat.unwrap();
    cbt_wrapper(func, ev, src, dst, skillname, id, revision)
}

unsafe fn cbt_wrapper_local(
    ev: *mut cbtevent,
    src: *mut u_ag,
    dst: *mut u_ag,
    skillname: PCCHAR,
    id: u64,
    revision: u64,
) -> usize {
    let funcs = FUNCTIONS.get().unwrap();
    let func = funcs.combat_local.unwrap();
    cbt_wrapper(func, ev, src, dst, skillname, id, revision)
}

unsafe fn cbt_wrapper(
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
    let s_skillname: Option<&'static str>;
    let p_src;
    let p_dst;

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

    func(s_ev, s_src, s_dst, s_skillname, id, revision);
    0
}

unsafe fn imgui_wrapper(not_charsel_or_loading: u32) -> usize {
    let funcs = FUNCTIONS.get().unwrap();
    let func = funcs.imgui.unwrap();
    func(not_charsel_or_loading != 0);
    0
}

unsafe fn get_safe_ag(ag: &u_ag) -> Ag {
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

// it is not necessarily static
// delta confirmed that skillnames are available for the whole lifetime of the plugin
// reduce the lifetime in the ongoing process as needed!
unsafe fn get_str_from_pcchar(src: PCCHAR) -> Option<&'static str> {
    if src.is_null() {
        None
    } else {
        Some(
            CStr::from_ptr(src as *const c_char)
                .to_str()
                .unwrap_or_default(),
        )
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

unsafe impl Send for arcdps_exports {}
unsafe impl Sync for arcdps_exports {}

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

#[derive(Debug, Clone)]
pub struct AgOwned {
    pub name: Option<String>,
    pub id: usize,
    pub prof: u32,
    pub elite: u32,
    pub self_: u32,
    pub team: u16,
}

impl From<Ag<'_>> for AgOwned {
    fn from(ag: Ag<'_>) -> Self {
        AgOwned {
            name: ag.name.map(|x| x.to_string()),
            id: ag.id,
            prof: ag.prof,
            elite: ag.elite,
            self_: ag.self_,
            team: ag.team,
        }
    }
}