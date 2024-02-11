use crate::{widget::HWidget, widgets::label_widget::LabelWidget};
use gtk::{Label, Revealer, RevealerTransitionType};

/// Bar configuration
pub const COLORS: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 1.0);
pub const POSITION: Position = Position::TOP;
pub const EXPAND_RIGHT: bool = true;
pub const EXPAND_LEFT: bool = true;
pub const UPDATE_RATE: u64 = 100;

pub fn get_widgets() -> (Widgets, Widgets, Widgets) {
    (
        vec![(
            "label",
            Box::new(LabelWidget {
                tooltip: "hi".to_string(),
                tooltip_command: "echo hi".to_string(),
                text: "lorem ipsum".to_string(),
                command: "".to_string(),
                update_rate: 10,
                label: Label::new(None),
                listen: false,
                revealer: Revealer::new(),
                update_anim: Some(RevealerTransitionType::Crossfade),
                anim_duration: 250,
            }),
        )],
        vec![],
        vec![],
    )
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Position {
    TOP,
    BOTTOM,
}

pub type Widgets = Vec<(&'static str, Box<dyn HWidget>)>;

/// Returns the set update-rate.
pub fn get_update_rate() -> u64 {
    UPDATE_RATE
}
