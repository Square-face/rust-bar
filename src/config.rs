use crate::widgets::label_widget::LabelWidget;
use gtk::{Label, Revealer, RevealerTransitionType};

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
                update_anim: Some(RevealerTransitionType::Crossfade),
                anim_duration: 250,
            },
        )],
        centered: vec![],
        right: vec![],
    }
}

pub const UPDATE_RATE: u64 = 100;

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

/// Returns the set update-rate.
pub fn get_update_rate() -> u64 {
    UPDATE_RATE
}
