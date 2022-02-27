use log::{Metadata, Record};

use crate::{e3, e8};

pub(crate) struct ArcdpsLogger {
    name: &'static str,
}

impl ArcdpsLogger {
    pub(crate) fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl log::Log for ArcdpsLogger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        ArcdpsFileLogger::log(&ArcdpsFileLogger { name: self.name }, record);
        ArcdpsWindowLogger::log(&ArcdpsWindowLogger { name: self.name }, record);
    }

    fn flush(&self) {}
}

struct ArcdpsFileLogger {
    name: &'static str,
}

impl log::Log for ArcdpsFileLogger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        let body = format!(
            "{} - {}:{} {}: {}\0",
            self.name,
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default(),
            record.level(),
            record.args(),
        );
        unsafe { e3(body.as_ptr() as _) };
    }

    fn flush(&self) {}
}

struct ArcdpsWindowLogger {
    name: &'static str,
}

impl log::Log for ArcdpsWindowLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.target() == "window"
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let body = format!(
            "{} - {}:{} {}: {}\0",
            self.name,
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default(),
            record.level(),
            record.args(),
        );
        unsafe { e8(body.as_ptr() as _) };
    }

    fn flush(&self) {}
}
