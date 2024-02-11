use crate::{
    constants::ERR_EMPTY_NAME,
    structures::BaseKeys,
    ui,
    widget::{Align, HWidget},
    WidgetType,
};
use gtk::{traits::*, *};
use json::JsonValue;

/// Creates a new basic box widget.
pub struct BoxWidget {
    pub width: i32,
    pub widgets: Vec<WidgetType>,
}

/// Builds the child widgets.
fn build_child_widgets(
    widgets: Vec<WidgetType>,
    left: &Box,
    centered: &Box,
    right: &Box,
    box_holder: &Box,
) {
    for widget in widgets {
        match widget {
            WidgetType::Label(name, label) => {
                label.add(name, Align::Left, left, centered, right, Some(box_holder))
            }
        }
    }
}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for BoxWidget {
    fn add(
        self,
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
        build_child_widgets(self.widgets, left, centered, right, &widget);

        ui::add_and_align(&widget, align, left, centered, right, box_holder);
        log!("Added a new box widget");
    }
}
