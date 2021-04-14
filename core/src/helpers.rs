use crate::*;
use std::ffi::CStr;

/// A helper function to convert raw arguments to safe abstractions
pub fn get_combat_args_from_raw<'a>(
    raw_ev: *mut CombatEvent,
    raw_src: *mut RawAgent,
    raw_dst: *mut RawAgent,
    raw_skill_name: PCCHAR,
) -> CombatEventArgs<'a> {
    let mut args = CombatEventArgs {
        ev: None,
        src: None,
        dst: None,
        skill_name: None,
    };

    unsafe {
        if !raw_ev.is_null() {
            args.ev = Some(&*raw_ev)
        }
        if !raw_src.is_null() {
            args.src = Some(raw_src.into());
        }
        if !raw_dst.is_null() {
            args.dst = Some(raw_dst.into());
        }
        args.skill_name = get_str_from_pc_char(raw_skill_name);
    };
    args
}

/// A helper function to convert arcdps strings to [`&str`].
/// ### Remarks
/// The result is not necessarily static.
/// delta confirmed that skill names are available for the whole lifetime of the
/// plugin, but agent names are only available for the duration of the fight.
/// Reduce the lifetime in the ongoing process as needed!
pub(crate) unsafe fn get_str_from_pc_char(src: PCCHAR) -> Option<&'static str> {
    if src.is_null() {
        None
    } else {
        Some(
            CStr::from_ptr(src as *const std::os::raw::c_char)
                .to_str()
                .unwrap_or_default(),
        )
    }
}

pub struct CombatEventArgs<'a> {
    pub ev: Option<&'a CombatEvent>,
    pub src: Option<Agent<'a>>,
    pub dst: Option<Agent<'a>>,
    pub skill_name: Option<&'static str>,
}
