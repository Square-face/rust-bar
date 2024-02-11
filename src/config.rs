use crate::{constants::*, structures::ConfigData, utils::environment};
use json::JsonValue;
use std::{
    collections::HashMap,
    fs,
    sync::{RwLock, RwLockReadGuard},
};

/// Bar configuration
pub const POSITION: Position = Position::TOP;
pub const EXPAND_RIGHT: bool = true;
pub const EXPAND_LEFT: bool = true;

pub const UPDATE_RATE: u64 = 100;
pub const CAVA_UPDATE_RATE: u64 = 1;
pub const CAVA_SED: &str = "s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;";
pub const CAVA_BARS: i32 = 5;
pub const CAVA_FRAMERATE: i32 = 60;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Position {
    TOP,
    BOTTOM,
}

lazy_static! {
    /// Cached config.
    pub static ref CONFIG: RwLock<JsonValue> = RwLock::new(read_config_raw());
}

/// Gets the root home path to Rust-Bar.
pub fn get_path() -> String {
    format!(
        "{}/.config/RustBar/",
        std::env::var("HOME").unwrap_or_else(|_| execute!("whoami"))
    )
}

/// Returns the set update-rate.
pub fn get_update_rate() -> u64 {
    UPDATE_RATE
}

// Parses and returns the config.
fn read_config_raw() -> JsonValue {
    let mut conf_path = get_path();
    conf_path.push_str(&environment::try_get_var("RUSTBAR_CONFIG", DEFAULT_CONFIG));
    json::parse(
        &fs::read_to_string(&conf_path)
            .unwrap(),
    )
    .unwrap_or_else(|error| panic!("[ERROR] Error parsing config: {error}"))
}

/// Returns the entire config.
pub fn get_config<'a>() -> RwLockReadGuard<'a, JsonValue> {
    CONFIG.read().expect(ERR_ACCESS_CONFIG)
}

/// Gets all the custom variables.
pub fn get_custom_variables() -> HashMap<String, String> {
    let cfg = &get_config()[RUSTBAR_V_ROOT_JSON];
    let mut map: HashMap<String, String> = HashMap::new();
    for entry in cfg.entries() {
        map.insert(entry.0.to_owned(), entry.1.to_string());
    }

    map
}

/// Replaces any variable-matching patterns in the `String` with the variables value.
pub fn with_variables(input: String, custom_variables: &HashMap<String, String>) -> String {
    let mut input = input;
    for variable in custom_variables {
        // Only replace if `result` actually contains the defined variable.
        if input.contains(variable.0) {
            input = input.replace(variable.0, variable.1);
        }
    }

    input
}
