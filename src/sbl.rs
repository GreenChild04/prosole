//! `SBL` stands for **Soulog Basic Loggers**
//! 
//! Some basic pre-made loggers for small, simple, applications.

use crate::*;

/// The default logger for just testing out this library, it just panics the second there is a (non-inconvenience) error
pub struct PanicLogger;
impl Logger for PanicLogger {
    fn new() -> Self { Self }
    fn hollow(&self) -> Self { Self }

    fn crash<T>(&mut self) -> T {
        panic!("program crashed (didn't exit successfully)")
    }

    fn verbose(&mut self, log: Log) {
        let log = colour_format![blue("["), cyan(log.origin), blue("] "), none(log.message)];
        println!("{log}");
    }

    fn vital(&mut self, log: Log) {
        let log = match log.log_type {
            LogType::Warning => colour_format![blue("["), yellow(log.origin), blue("] "), yellow("Warning: "), none(log.message)],
            LogType::Inconvenience => colour_format![blue("["), yellow(log.origin), blue("] "), yellow("Inconvenience: "), none(log.message)],
            LogType::Result => colour_format![blue("["), green("Result"), blue("] "), green(log.origin), blue(": "), none(log.message)],
            _ => panic!("meta error: invalid vital log type '{:?}'", log.log_type),
        }; println!("{log}");
    }

    fn error(&mut self, log: Log) -> ErrorResponse {
        let log = match log.log_type {
            LogType::Fatal => colour_format![blue("["), red(log.origin), blue("] "), red("Fatal: "), none(log.message)],
            LogType::Failure => colour_format![blue("["), red(log.origin), blue("] "), red("Failure: "), none(log.message)],
            _ => panic!("meta error: invalid error log type '{:?}'", log.log_type),
        }; println!("{log}");
        ErrorResponse::Panic
    }
}