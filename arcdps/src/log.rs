//! Logging via the [`log`] crate.
//!
//! # Usage
//! With the `"log"` feature enabled, [`ArcDpsLogger`] is set as logger before your `init` function is called.
//! By default all messages are logged to ArcDPS' log window and only warnings & errors are logged to the log file.
//! You can specify `"window"`, `"file"` or `"both"`/`"all"` as log target to control where the messages should be logged.
//!
//! ```no_run
//! use log::{error, info};
//!
//! error!("an error will log to window & file");
//! error!(target: "window", "window target will only log to window");
//! error!(target: "file", "file target will only log to file");
//! info!("below error/warn level will only log to window");
//! info!(target: "both", "target both/all will log to window & file");
//! ```
//!
//! *Requires the `"log"` feature.*

use crate::exports::{log_to_file, log_to_window};
use log::{Level, Log, Metadata, Record};

/// A logger logging to ArcDPS' log window and/or file.
///
/// By default all messages are logged to ArcDPS' log window and only warnings & errors are logged to the log file.
/// You can specify `"window"`, `"file"` or `"both"`/`"all"` as log target to control where the messages should be logged.
#[derive(Debug, Clone)]
pub struct ArcDpsLogger {
    name: &'static str,
}

impl ArcDpsLogger {
    /// Creates a new ArcDPS logger.
    #[inline]
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    /// Checks whether window logging is enabled for the given [`Metadata`].
    fn window_enabled(metadata: &Metadata) -> bool {
        metadata.target() != "file"
    }

    /// Checks whether file logging is enabled for the given [`Metadata`].
    fn file_enabled(metadata: &Metadata) -> bool {
        match metadata.target() {
            "file" | "both" | "all" => true,
            "window" => false,
            _ => matches!(metadata.level(), Level::Warn | Level::Error),
        }
    }
}

impl Log for ArcDpsLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        Self::window_enabled(metadata) || Self::file_enabled(metadata)
    }

    fn log(&self, record: &Record) {
        let metadata = record.metadata();
        if Self::window_enabled(metadata) {
            WindowLogger { name: self.name }.log(record);
        }
        if Self::file_enabled(metadata) {
            FileLogger { name: self.name }.log(record);
        }
    }

    fn flush(&self) {}
}

/// A logger logging to ArcDPS' log window.
#[derive(Debug, Clone)]
pub struct WindowLogger {
    name: &'static str,
}

impl WindowLogger {
    /// Creates a new window logger.
    #[inline]
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Log for WindowLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // TODO: coloring?
        let _ = log_to_window(format_message(self.name, record));
    }

    fn flush(&self) {}
}

/// A logger logging to ArcDPS' log file.
#[derive(Debug, Clone)]
pub struct FileLogger {
    name: &'static str,
}

impl FileLogger {
    /// Creates a new file logger.
    #[inline]
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Log for FileLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let _ = log_to_file(format_message(self.name, record));
    }

    fn flush(&self) {}
}

/// Formats a log message.
fn format_message(name: &'static str, record: &Record) -> String {
    format!(
        "{} {}: {}",
        name,
        record.level().to_string().to_lowercase(),
        record.args()
    )
}

#[cfg(test)]
mod tests {
    use super::ArcDpsLogger;
    use log::{Level, Metadata};

    #[test]
    fn enabled() {
        fn meta(target: &str, level: Level) -> Metadata<'_> {
            Metadata::builder().target(target).level(level).build()
        }

        const MOD: &str = module_path!();

        let info = meta(MOD, Level::Info);
        assert!(ArcDpsLogger::window_enabled(&info));
        assert!(!ArcDpsLogger::file_enabled(&info));

        let warn = meta(MOD, Level::Warn);
        assert!(ArcDpsLogger::window_enabled(&warn));
        assert!(ArcDpsLogger::file_enabled(&warn));

        let error = meta(MOD, Level::Error);
        assert!(ArcDpsLogger::window_enabled(&error));
        assert!(ArcDpsLogger::file_enabled(&error));

        let info_file = meta("file", Level::Info);
        assert!(!ArcDpsLogger::window_enabled(&info_file));
        assert!(ArcDpsLogger::file_enabled(&info_file));

        let info_both = meta("both", Level::Info);
        assert!(ArcDpsLogger::window_enabled(&info_both));
        assert!(ArcDpsLogger::file_enabled(&info_both));
    }
}
