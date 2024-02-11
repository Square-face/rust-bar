#![no_main]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

mod config;
mod constants;
mod r#loop;
mod structures;
mod ui;
mod utils;
mod widget;
mod widgets;

use config::*;
use constants::*;
use gtk::gdk::*;
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::*;
use gtk_layer_shell::Edge;
use json::JsonValue;

fn get_anchors() -> [(gtk_layer_shell::Edge, bool); 4] {
    [
        (Edge::Left, EXPAND_LEFT),
        (Edge::Right, EXPAND_RIGHT),
        (Edge::Top, POSITION == Position::TOP),
        (Edge::Bottom, POSITION == Position::BOTTOM),
    ]
}

/// Initializes the status bar.
fn activate(application: &Application) {
    // Create a normal GTK window however you like
    let window = ApplicationWindow::new(application);
    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    // Initialize layer shell before the window has been fully initialized.
    gtk_layer_shell::init_for_window(&window);

    // Order above normal windows
    // Prior to 0.2.9, this was set to Bottom but it caused issues with tooltips being shown below
    // windows.
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Top);

    // Push other windows out of the way
    // Toggling this off may help some if they are in applications that have weird unicode text, which may mess with the bars scaling.
    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    for (anchor, state) in get_anchors() {
        gtk_layer_shell::set_anchor(&window, anchor, state);
    }

    // Allows for specifying the namespace of the layer.
    // The default is "gtk-layer-shell" to not break existing configs.
    gtk_layer_shell::set_namespace(&window, "gtk-layer-shell");

    // Initialize gdk::Display by default value, which is decided by the compositor.
    let display = Display::default().expect(ERR_GET_DISPLAY);

    // Loads the monitor variable from config, default is 0.
    let config_monitor = 0;

    // Gets the actual gdk::Monitor from configured number.
    let monitor = display.monitor(config_monitor).expect(ERR_GET_MONITOR);

    // Sets which monitor should be used for the bar.
    gtk_layer_shell::set_monitor(&window, &monitor);

    // For transparency to work.
    window.set_app_paintable(true);

    // Build all the widgets.
    ui::build_widgets(&window);
    log!("Ready!");
}

/// Loads the CSS
pub fn load_css() {
    let provider = CssProvider::new();

    provider
        .load_from_data(include_bytes!("../style.css"))
        .expect(ERR_LOAD_SAMPLE_CSS);

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        &Screen::default().expect(ERR_SCREEN_DEFAULT),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Called upon application startup.
#[no_mangle]
#[tokio::main]
async fn main() {
    log!("Building application...");
    let application = Application::new(None, ApplicationFlags::default());
    log!("Loading CSS...");
    application.connect_startup(|_| load_css());
    log!("Creating viewport...");
    // Activate the layer shell.
    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}

/// Applies custom visuals.
fn set_visual(window: &ApplicationWindow, screen: Option<&Screen>) {
    if let Some(screen) = screen {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // Needed for transparency, not available in GTK 4+ so
                                             // F.
        }
    }
}

/// Converts the value of a child inside `background` to a `f64`.
fn get_background_float(cfg: &JsonValue, identifier: &str, from_255: bool) -> f64 {
    let mut res = cfg[RUSTBAR_ROOT_JSON][identifier]
        .as_f64()
        .unwrap_or_else(|| panic!("[ERROR] Failed converting hybrid:{identifier} to f64!"));

    // Only divide by 255 if explicitly told to.
    if from_255 {
        res /= 255.0;
    }

    // Return the result.
    res.clamp(0.0, 255.0)
}

/// Draws the window using a custom color and opacity.
fn draw(_: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    // Apply
    ctx.set_source_rgba(COLORS.0, COLORS.1, COLORS.2, COLORS.3);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect(ERR_CUSTOM_DRAW);
    Inhibit(false)
}
