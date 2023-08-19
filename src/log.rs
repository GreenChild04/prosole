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
    Inconvenience,
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

/// Logs to a logger
/// # Usage
/// ```rust
/// use soulog::*;
/// fn process(mut logger: impl Logger) {
///     log!((logger) Process("Example log of number: {}", 12));
///     log!((logger.error) Process("Example inconvenience log") as Inconvenience);
/// }
/// ```
#[macro_export]
macro_rules! log {
    (($logger:ident) $origin:ident$tokens:tt) => {
        $logger.log(
            Log::new(LogType::Log, stringify!($origin), &format!$tokens, &[])
        );
    };

    (($logger:ident) $origin:ident$tokens:tt as $type:ident) => {
        $logger.log(
            Log::new(LogType::$type, stringify!($origin), &format!$tokens, &[])
        );
    };

    (($logger:ident.$func:ident) $origin:ident$tokens:tt as $type:ident) => {
        $logger.$func(
            Log::new(LogType::$type, stringify!($origin), &format!$tokens, &[])
        );
    };
}

/// Checks if a result is ok or not, if not, then it will log an error and react to the response.
/// # Usage
/// For code the can be retried *eg* an io error:
/// `if_err!((logger) [Process, <err_var> => "Error Message"] retry {<code that returns a result>})`
/// 
/// For errors that can be easily recovered from:
/// `if_err!((logger) {<code the returns result>} else(<error varible name>) {<code that handles the error>})`
/// 
/// For completely custom `ErrorResponse` response:
/// `if_err!((logger) [Process, <err_var> => "Error Message"] {<result returning code>} manual {<ErrorResponse> => <code that handles it>})`
#[macro_export]
macro_rules! if_err {
    (($logger:ident) [$origin:ident, $err:ident => $msg:tt] retry $code:expr) => {
        loop { match $code {
            Ok(x) => break x,
            Err($err) => match $logger.error(
                Log::new(LogType::Failure, stringify!($origin), &format!$msg, &[ErrorResponse::Retry, ErrorResponse::Crash, ErrorResponse::Panic]),
            ) {
                ErrorResponse::Retry => continue,
                ErrorResponse::Panic => panic!("unexpected error: {:#?}", $err),
                ErrorResponse::Crash => {
                    let _ = $logger.error(Log::new(LogType::Fatal, stringify!($origin), &format!("{:#?}", $err), &[]));
                    $logger.crash()
                },
                ErrorResponse::AskUser => panic!("meta error: logger returned invalid error response"),
            }
        }}
    };

    (($logger:ident) $code:block else($err:ident) $else:block) => {
        match $code {
            Ok(x) => x,
            Err($err) => $else,
        }
    };

    (($logger:ident) [$origin:ident, $err:ident => $msg:tt] $code:block manual {
        $($response:ident => $action:expr),* $(,)?
    }) => {
        match $code {
            Ok(x) => x,
            Err($err) => match $logger.error(
                Log::new(LogType::Failure, stringify!($origin), &format!$msg, &[$(ErrorResponse::$response),*])
            ) {
                $(ErrorResponse::$response => $action,)*
                _ => panic!("meta error: logger returned invalid error response"),
            }
        }
    }
}