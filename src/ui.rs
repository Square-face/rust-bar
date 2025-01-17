use crate::{r#loop::update, *};
use gtk::traits::*;

/// Builds all of the widgets.
pub fn build_widgets(window: &ApplicationWindow) {
    // Create box widgets, which we'll be using to draw the content onto.
    let root = Box::new(Orientation::Horizontal, 0);
    let left = Box::new(Orientation::Horizontal, 0);
    let centered = Box::new(Orientation::Horizontal, 0);
    let right = Box::new(Orientation::Horizontal, 0);

    // 0.2.5: Root expands across the entire bar, previously "left" would do this but it isn't
    //   ideal when customizing, since borders would draw on the entire bar rather than just on the
    //   left portion of the bar.
    root.set_widget_name("root");

    // 0.2.5: Allow for customizing left, centered and right.
    left.set_widget_name("left");
    centered.set_widget_name("centered");
    right.set_widget_name("right");

    root.set_center_widget(Some(&centered));
    root.pack_end(&right, false, true, 0);
    root.add(&left);
    window.add(&root);

    // Prepare and show all of the widgets.
    create_components(&left, &centered, &right);
    window.show_all();

    // Update dynamic content.
    update();
}

/// Creates all of the widgets.
fn create_components(left: &Box, centered: &Box, right: &Box) {
    let widgets = get_widgets();

    for mut widget in widgets.0 {
        widget.1.add(widget.0, left)
    }

    for mut widget in widgets.1 {
        widget.1.add(widget.0, centered)
    }

    for mut widget in widgets.2 {
        widget.1.add(widget.0, right)
    }
}
