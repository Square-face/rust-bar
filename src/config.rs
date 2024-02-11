use crate::{
    constants::*, structures::RevealerExtensions, utils::environment,
    widgets::label_widget::LabelWidget,
};
use gtk::{Label, Revealer, RevealerTransitionType};
use json::JsonValue;
use std::{
    collections::HashMap,
    fs,
    sync::{RwLock, RwLockReadGuard},
};

/// Bar configuration
pub const COLORS: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 1.0);
pub const POSITION: Position = Position::TOP;
pub const EXPAND_RIGHT: bool = true;
pub const EXPAND_LEFT: bool = true;

pub fn get_widgets() -> Widgets {
    Widgets {
        left: vec![WidgetType::Label(
            "lorem",
            LabelWidget {
                tooltip: "hi".to_string(),
                tooltip_command: "echo hi".to_string(),
                text: "lorem ipsum".to_string(),
                command: "echo lorem ipsum".to_string(),
                update_rate: 10,
                label: Label::new(None),
                listen: false,
                revealer: Revealer::new(),
                update_anim: RevealerTransitionType::from_str("crossfade"),
                anim_duration: 250,
            },
        )],
        centered: vec![],
        right: vec![],
    }
}

pub const UPDATE_RATE: u64 = 100;
pub const CAVA_UPDATE_RATE: u64 = 1;
pub const CAVA_SED: &str =
    "s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;";
pub const CAVA_BARS: i32 = 5;
pub const CAVA_FRAMERATE: i32 = 60;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Position {
    TOP,
    BOTTOM,
}

pub struct Widgets {
    pub left: Vec<WidgetType>,
    pub centered: Vec<WidgetType>,
    pub right: Vec<WidgetType>,
}

pub enum WidgetType {
    Label(&'static str, LabelWidget),
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
