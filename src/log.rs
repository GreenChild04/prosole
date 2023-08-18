/// Different Types of Logs
/// 
/// **`Log`:** A normal log *eg* `Loaded version 1.2.0`
/// 
/// **`Inconvienience`:** An expected recoverable error *eg* `Database not found; creating a new one`
/// 
/// **`Failure`:** An unexpected error that could be recoverable *eg* `IO error; retrying...`
/// 
/// **`Fatal`:** A major error that is unrecoverable and forces the application to crash *eg* `Everything that could've gone wrong has gone wrong; crashing app`
pub enum LogType {
    /// A normal log *eg* `Loaded version 1.2.0`
    Log,
    /// An expected recoverable error *eg* `Database not found; creating a new one`
    Inconvienience,
    /// An unexpected error that could be recoverable *eg* `IO error; retrying...`
    Failure,
    /// A major error that is unrecoverable and forces the application to crash *eg* `Everything that could've gone wrong has gone wrong; crashing app`
    Fatal,
}

/// A log to be passed into a `Logger`
pub struct Log<'a> {
    /// Type of the log *eg* `Log`
    pub log_type: LogType,
    /// The function / process the log originates from *eg* `Password Manager`
    pub origin: &'a str,
    /// The actual log itself *eg* `Collected password is correct!`
    pub message: &'a str,
    /// Allowed responses from a `Logger`
    pub allowed_responses: &'a [ErrorResponse],
}

/// Possible responses from a `Logger` during an error
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ErrorResponse {
    /// Asks the process to retry the action in hopes it works on the second try
    Retry,
    /// Asks the process to just panic
    Panic,
    /// Asks the process to crash the application gracefully
    Crash,
    /// Asks the process to ask the user to recover from error
    AskUser,
}

impl ErrorResponse {
    /// Check if response is allowed for the log.
    #[inline]
    pub fn allowed_in(&self, log: &Log) -> bool {
        log.allowed_responses.contains(self)
    }
}

impl<'a> Log<'a> {
    /// Constructs a new log
    pub fn new(log_type: LogType, origin: &'a str, message: &'a str, allowed_responses: &'a [ErrorResponse]) -> Self {
        Self {
            log_type,
            origin,
            message,
            allowed_responses,
        }
    }
}