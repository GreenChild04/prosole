/// Defines the escape sequences required for each wanted colour
pub struct Colours;

macro_rules! def_colours {
    ($(
        $name:ident = $colour:expr;
    )*) => {
        impl Colours {
            $(
                pub const $name: &'static str = $colour;
            )*
        }
    }
}

def_colours! {
    NORMAL = "\x1b[0m";
    PINK = "\x1b[35m";
    WHITE = "\x1b[37m";
    BLUE = "\x1b[34m";
    GREEN = "\x1b[32m";
    CYAN = "\x1b[36m";
    RED = "\x1b[31m";
    BLACK = "\x1b[30m";
    YELLOW = "\x1b[33m";
}