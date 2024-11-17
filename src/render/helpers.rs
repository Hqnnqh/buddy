use gtk4::prelude::NativeExt;
use gtk4::ApplicationWindow;
use gtk4::CssProvider;

use gdk4::cairo::{RectangleInt, Region};
use gdk4::prelude::{DisplayExt, MonitorExt, SurfaceExt};
use gdk4::Display;

/// Update click-able section of buddy on screen.
pub(super) fn update_input_region(window: &ApplicationWindow, character_size: i32, x: i32, y: i32) {
    let region = Region::create_rectangle(&RectangleInt::new(x, y, character_size, character_size));
    window.surface().unwrap().set_input_region(&region);
}

/// Returns the screen resolution (width, height). May fail and return None.
pub(super) fn screen_resolution(window: &ApplicationWindow) -> Option<(i32, i32)> {
    let display = Display::default()?;

    let monitor = display.monitor_at_surface(&window.surface()?)?;
    Some((monitor.geometry().width(), monitor.geometry().height()))
}

/// Make buddy's background transparent.
pub(super) fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(
        r#"* {
        background-color: transparent;
    }"#,
    );

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    )
}
