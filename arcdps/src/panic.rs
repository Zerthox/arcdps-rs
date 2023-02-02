//! Panic handling.

use crate::exports::log_to_file;
use std::panic;

/// Sets up the custom panic hook.
// TODO: rust will abort when reaching the ffi boundary, which skips arcs crash log
pub fn init_panic_hook(name: &'static str) {
    panic::set_hook(Box::new(move |info| {
        // log the error to the arc log
        let _ = log_to_file(format!("error: {name} {info}"));
    }));
}
