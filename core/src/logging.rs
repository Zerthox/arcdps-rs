use crate::{e3, e8};
use log::{Metadata, Record};

pub(crate) static LOGGER: ArcdpsLogger = ArcdpsLogger;

pub(crate) struct ArcdpsLogger;

impl log::Log for ArcdpsLogger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        ArcdpsFileLogger::log(&ArcdpsFileLogger, record);
        ArcdpsWindowLogger::log(&ArcdpsWindowLogger, record);
    }

    fn flush(&self) {}
}

struct ArcdpsFileLogger;

impl log::Log for ArcdpsFileLogger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        let body = format!(
            "{}:{} {}: {}\0",
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default(),
            record.level(),
            record.args(),
        );
        unsafe { e3(body.as_ptr() as _) };
    }

    fn flush(&self) {}
}

struct ArcdpsWindowLogger;

impl log::Log for ArcdpsWindowLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.target() == "window"
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let body = format!(
            "{}:{} {}: {}\0",
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default(),
            record.level(),
            record.args(),
        );
        unsafe { e8(body.as_ptr() as _) };
    }

    fn flush(&self) {}
}
