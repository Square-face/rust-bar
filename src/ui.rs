use crate::widget::{Align, HWidget};
use crate::{r#loop::update, *};
use gtk::traits::*;

/// Adds and aligns the specified widget.
pub fn add_and_align(
    widget: &impl IsA<Widget>,
    align: Align,
    left: &Box,
    centered: &Box,
    right: &Box,
    box_holder: Option<&Box>,
) {
    if let Some(r#box) = box_holder {
        r#box.add(widget)
    } else {
        match align {
            Align::Left => left.add(widget),
            Align::Centered => centered.add(widget),
            Align::Right => right.add(widget),
        }
    }
}

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
    for widget in get_widgets().left {
        match widget {
            WidgetType::Label(name, label) => {
                label.add(name, Align::Left, left, centered, right, None)
            }
        }
    }

    for widget in get_widgets().centered {
        match widget {
            WidgetType::Label(name, label) => {
                label.add(name, Align::Centered, left, centered, right, None)
            }
        }
    }

    for widget in get_widgets().right {
        match widget {
            WidgetType::Label(name, label) => {
                label.add(name, Align::Right, left, centered, right, None)
            }
        }
    }
}
