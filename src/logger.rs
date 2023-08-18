use crate::*;

/// Logger that logs stuff; not that complicated.
pub trait Logger {
    /// Creates a new instance of the logger
    fn new() -> Self;
    /// Creates a new instance of the logger (for the impl syntax)
    fn hollow(&self) -> Self;
    /// Crashes the application gracefully
    fn crash<T>(&self) -> T { std::process::exit(1) }
    /// Logs an error to the console and informs process how to recover
    fn error(&mut self, log: Log) -> ErrorResponse;

    /// Logs a log to console
    fn log(&self, log: Log) {
        let log = colour_format![blue("["), cyan(log.origin), blue("] "), none(log.message)];
        println!("{log}");
    }

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