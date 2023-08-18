# soulog
A library for polished, clean and colourful console logging and error handling.

# Colourful Printing
---
Some utils for printing to the console in colour.
- ## Supported Colours
    - None
    - Pink
    - White
    - Blue
    - Green
    - Cyan
    - Red
    - Black
    - Yellow

- ## Usage
    ```rust
    use soulog::*;
    
    // The macro is pretty simple, it's just a list of strings and you have to surround each string with a colour (in lowercase)
    // If you don't want a colour, then use `none` for the default colour of the console.
    let colourful_text = colour_format![blue("["), none("Example"), blue("] "), none("Here is an example body!")];
    
    println!("{}", colourful_text); // Prints the colourful text to screen
    ```

# Loggers
---
- A struct that determines what to do with logs and errors, and how to handle them.
- The default logger for smaller applications is `soulog::sbl::PanicLogger` which panics on errors

# Logging
---
For when you want to log an event or just to give an update to the user through the console
```rust
use soulog::*;

pub fn process(mut logger: impl Logger) {
    // First part is the logger surrounded by parenthesis, this is the destination the log is going to
    // The second part is the origin of the log, the function is called `process` so it should be `Process`
    // The third part is the log body, it should be formatted like in a `format!()` macro
    log!((logger) Process("Example log of a number: {}", 12));
    
    // You can also have error-like logs such as inconvenience logs
    log!((logger.error) Process("Example inconvenience log") as Inconvenience);
}
```