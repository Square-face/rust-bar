use crate::widget::HWidget;
use gtk::{traits::*, *};

/// Creates a new basic spacing widget.
pub struct SpacingWidget {
    pub spacing_start: i32,
    pub spacing_end: i32,
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for SpacingWidget {
    fn add(&mut self, name: &str, spot: &Box) {
        let widget = Box::new(Orientation::Horizontal, 0);
        // 0.2.2: Allow for named spacings
        widget.set_widget_name(name);
        widget.set_margin_start(self.spacing_start);
        widget.set_margin_end(self.spacing_end);

        spot.add(&widget);
        log!("Added a new spacing widget");
    }
}
