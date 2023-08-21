//! `SBL` stands for **Soulog Basic Loggers**
//! 
//! Some basic pre-made loggers for small, simple, applications.

use crate::*;

/// The default logger for just testing out this library, it just panics the second there is a (non-inconvenience) error
pub struct PanicLogger;
impl Logger for PanicLogger {
    fn new() -> Self { Self }
    fn hollow(&self) -> Self { Self }

    fn crash<T>(&self) -> T {
        panic!("program crashed (didn't exit successfully)")
    }

    fn error(&mut self, log: Log) -> ErrorResponse {
        let log = match log.log_type {
            LogType::Inconvenience => colour_format![blue("["), yellow(log.origin), blue("] "), yellow("Inconvenience: "), none(log.message)],
            LogType::Fatal => colour_format![blue("["), red(log.origin), blue("] "), red("Fatal: "), none(log.message)],
            LogType::Failure => colour_format![blue("["), red(log.origin), blue("] "), red("Failure: "), none(log.message)],
            LogType::Log => colour_format![blue("["), red(log.origin), blue("] "), none(log.message)],
        }; println!("{log}");
        ErrorResponse::Panic
    }
}