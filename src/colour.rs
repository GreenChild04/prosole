pub mod colours;
pub use colours::Colours;
pub use crate::colour_format;

/// Used to format text with colour
/// # Examples
/// ```rust
/// colour_format![pink("["), none("Logger"), pink("] "), none("Example Log")]
/// // outputs: [Logger] Example Log
/// // but with colour
/// ```
#[macro_export]
macro_rules! colour_format { // Verbose ugly stuff I can't read
    ($(
        $(none($none:expr))?
        $(blue($blue:expr))?
        $(pink($pink:expr))?
        $(white($white:expr))?
        $(green($green:expr))?
        $(cyan($cyan:expr))?
        $(red($red:expr))?
        $(black($black:expr))?
        $(yellow($yellow:expr))?
    ),*) => {{
        let mut string = String::new();
        $(
            $(string.push_str(Colours::NORMAL); string.push_str($none);)?
            $(string.push_str(Colours::BLUE); string.push_str($blue);)?
            $(string.push_str(Colours::PINK); string.push_str($pink);)?
            $(string.push_str(Colours::WHITE); string.push_str($white);)?
            $(string.push_str(Colours::GREEN); string.push_str($green);)?
            $(string.push_str(Colours::CYAN); string.push_str($cyan);)?
            $(string.push_str(Colours::RED); string.push_str($red);)?
            $(string.push_str(Colours::BLACK); string.push_str($black);)?
            $(string.push_str(Colours::YELLOW); string.push_str($yellow);)?
        )* string.push_str(Colours::NORMAL);
        string
    }}
}