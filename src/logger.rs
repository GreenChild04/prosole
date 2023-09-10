use crate::*;

/// Logger that logs stuff; not that complicated.
pub trait Logger {
    /// Creates a new instance of the logger
    fn new() -> Self;
    /// Creates a new instance of the logger (for the impl syntax)
    fn hollow(&self) -> Self;
    /// Crashes the application gracefully
    fn crash<T>(&mut self) -> T { std::process::exit(1) }
    /// Logs an error to the console and informs process how to recover
    fn error(&mut self, log: Log) -> ErrorResponse;
    /// Logs vital logs to the console
    fn vital(&mut self, log: Log);

    /// Logs verbose logs to the console
    fn verbose(&mut self, log: Log);

    /// Asks user a question
    fn ask(&self, origin: &str, prompt: &str) -> String {
        use std::io::{self, Write};

        let prompt = colour_format![blue("["), cyan(origin), blue("] "), cyan(prompt), blue(": ")];
        let mut read_line = String::new();

        print!("{prompt}");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut read_line);

        read_line
    }
}