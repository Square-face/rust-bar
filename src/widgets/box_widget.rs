use crate::{
    ui,
    widget::{Align, HWidget},
    Widgets,
};
use gtk::{traits::*, *};

/// Creates a new basic box widget.
pub struct BoxWidget {
    pub width: i32,
    pub widgets: Widgets,
}

/// Builds the child widgets.
fn build_child_widgets(
    widgets: &mut Widgets,
    left: &Box,
    centered: &Box,
    right: &Box,
    _box_holder: &Box,
) {

    widgets.iter_mut().for_each(|widget| {
        widget.1.add(&widget.0, Align::Right, left, centered, right, None);
    })
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for BoxWidget {
    fn add(
        &mut self,
        name: &str,
        align: Align,
        left: &Box,
        centered: &Box,
        right: &Box,
        box_holder: Option<&Box>,
    ) {
        let widget = Box::new(Orientation::Horizontal, 0);
        widget.set_widget_name(name);
        widget.set_width_request(self.width);

        // 0.4.3: Experimental: Allow for widgets enclosed into boxes.
        // 0.4.7: Stabilize Box Child-Widgets.
        build_child_widgets(&mut self.widgets, left, centered, right, &widget);

        ui::add_and_align(&widget, align, left, centered, right, box_holder);
        log!("Added a new box widget");
    }
}
