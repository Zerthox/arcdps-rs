//! Logging via the [`log`] crate.
//!
//! *Requires the `"log"` feature.*

use crate::exports::{log_to_file, log_to_window};
use log::{Log, Metadata, Record};

pub(crate) struct ArcdpsLogger {
    name: &'static str,
}

impl ArcdpsLogger {
    pub(crate) fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Log for ArcdpsLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        FileLogger::log(&FileLogger { name: self.name }, record);
        WindowLogger::log(&WindowLogger { name: self.name }, record);
    }

    fn flush(&self) {}
}

/// A logger logging to ArcDPS' log window.
struct WindowLogger {
    name: &'static str,
}

impl Log for WindowLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.target() != "file" // by default log to window
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        // TODO: coloring
        let message = format!(
            "{} {}: {}",
            self.name,
            record.level().to_string().to_lowercase(),
            record.args()
        );
        let _ = log_to_window(message);
    }

    fn flush(&self) {}
}

/// A logger logging to ArcDPS' log file.
struct FileLogger {
    name: &'static str,
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        ["file", "both"].contains(&metadata.target())
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let message = format!(
            "{} {}: {}",
            self.name,
            record.level().to_string().to_lowercase(),
            record.args()
        );
        let _ = log_to_file(message);
    }

    fn flush(&self) {}
}
