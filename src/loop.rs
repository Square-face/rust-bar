use crate::{
    constants::{
        ERR_ACCESS_CAVA_INSTANCES, WARN_CAVA_NO_BARS_INSTANCE, WARN_CAVA_NO_CRASHED_INSTANCE,
    },
    utils::cava::{self, HAS_CAVA_CRASHED},
    widget::HWidget,
    CAVA_UPDATE_RATE,
};
use glib::Continue;
use std::time::Duration;

/// Updates dynamic bar content.
pub fn update() {
    // Only start the tick-loop if there are actually Cava widgets available.
    let widgets = cava::CAVA_INSTANCES
        .lock()
        .expect(ERR_ACCESS_CAVA_INSTANCES);
    if widgets.is_empty() {
        return;
    }

    // Run the `update_cava` closure every x ms.
    glib::timeout_add_local(Duration::from_millis(CAVA_UPDATE_RATE), update_cava);
}

/// Updates all Cava widgets.
fn update_cava() -> Continue {
    if let Ok(ref bars) = cava::BARS.lock() {
        // Loop through all Cava widget instances and sync the text.
        let widgets = cava::CAVA_INSTANCES
            .lock()
            .expect(ERR_ACCESS_CAVA_INSTANCES);
        let widgets = widgets.iter();
        for widget in widgets {
            widget.update_label_direct(bars);
        }

        if let Ok(has_cava_crashed) = HAS_CAVA_CRASHED.lock() {
            glib::Continue(!*has_cava_crashed)
        } else {
            log!(WARN_CAVA_NO_CRASHED_INSTANCE);
            glib::Continue(false)
        }
    } else {
        log!(WARN_CAVA_NO_BARS_INSTANCE);
        glib::Continue(false)
    }
}
