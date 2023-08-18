use crate::*;

/// Logger that logs stuff; not that complicated.
pub trait Logger {
    /// Creates a new instance of the logger
    fn new() -> Self;
    /// Creates a new instance of the logger (for the impl syntax)
    fn hollow(&self) -> Self;
    /// Logs a log to console
    fn log(&self, log: Log);
    /// Crashes the application gracefully
    fn crash<T>(&self, origin: &str, message: &str) -> T;
    /// Logs an error to the console and informs process how to recover
    fn error(&mut self, log: Log) -> ErrorResponse;
}