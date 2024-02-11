// This file is meant for holding constant strings / repeatedly-used numerical values.

/// Master Variables JSON Key.
pub const PROC_TARGET: &str = "sh";

// Constant errors.

pub const ERR_GET_DISPLAY: &str =
    "[ERROR] Couldn't find a valid display, is your compositor doing alright?";
pub const ERR_GET_MONITOR: &str = "[ERROR] Couldn't find a valid monitor.";
pub const ERR_SCREEN_DEFAULT: &str = "[ERROR] Couldn't find a valid screen!";
pub const ERR_LOAD_SAMPLE_CSS: &str = "[ERROR] Failed loading the example stylesheet!";
pub const ERR_CUSTOM_DRAW: &str =
    "[ERROR] Failed drawing Rust-bar using custom color sources, which is needed for transparency!";
pub const ERR_TAKE_STDOUT: &str = "[ERROR] Cannot take stdout from child process!";
pub const ERR_NO_LINES: &str = "[ERROR] There are no more lines available!";
pub const ERR_STRING_NONE: &str = "[ERROR] The string value is None!";
pub const ERR_WRONG_LABEL_RANIM: &str =
    "[ERROR] Invalid revealer animation! Use `crossfade`, `slide_left` or `slide_right`.";
