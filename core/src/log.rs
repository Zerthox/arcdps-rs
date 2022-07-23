//! Logging via the [`log`] crate.
//!
//! *Requires the `"log"` feature.*

use crate::exports::log_to_window;
use log::{Log, Metadata, Record};

/// A logger logging to ArcDPS' log window.
pub struct WindowLogger {
    name: &'static str,
}

impl WindowLogger {
    /// Creates a new window logger.
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Log for WindowLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // TODO: coloring
        let message = format!("{} {}: {}", self.name, record.level(), record.args());
        let _ = log_to_window(message);
    }

    fn flush(&self) {}
}
