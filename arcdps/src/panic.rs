//! Panic handling.

use crate::exports::log_to_file;
use std::{
    backtrace::Backtrace,
    ffi::CString,
    panic::{self, PanicHookInfo},
};
use windows::{
    core::PCSTR,
    Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR, MB_OK, MB_SETFOREGROUND},
};

/// Sets up the custom panic hook.
pub fn init_panic_hook(name: &'static str) {
    panic::set_hook(Box::new(move |info| {
        let message = if cfg!(feature = "panic_trace") {
            let trace = Backtrace::force_capture();
            format!("error: {name} {info}\n{trace:#}")
        } else {
            format!("error: {name} {info}")
        };

        let _ = log_to_file(message);

        #[cfg(feature = "panic_msgbox")]
        message_box(name, info);
    }));
}

fn message_box(name: &'static str, info: &PanicHookInfo) {
    let text = CString::new(format!("{name} {info}")).unwrap();
    let caption = CString::new(format!("{name} error")).unwrap();
    unsafe {
        MessageBoxA(
            None,
            PCSTR(text.as_ptr().cast()),
            PCSTR(caption.as_ptr().cast()),
            MB_OK | MB_ICONERROR | MB_SETFOREGROUND,
        )
    };
}
