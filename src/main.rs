use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

use gdk4::Display;
use gdk4::prelude::{DisplayExt, MonitorExt};
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use glib::{ControlFlow, timeout_add_local};
use gtk4::{ApplicationWindow, CssProvider, GestureClick};
use gtk4::prelude::{FixedExt, GtkWindowExt, NativeExt, WidgetExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

const CHARACTER_SIZE: i32 = 75;
const FPS: u32 = 4;
const MOVEMENT_SPEED: u32 = 20;

fn activate(application: &gtk4::Application) -> std::io::Result<()> {
    let window = gtk4::ApplicationWindow::new(application);

    window.init_layer_shell();
    // Display above normal windows
    window.set_layer(Layer::Overlay);


    for (anchor, state) in [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, false),
        (Edge::Bottom, true),
    ] {
        window.set_anchor(anchor, state);
    }

    window.present(); // present prematurely to be able to get screen resolution

    let screen_width = screen_width(&window).ok_or(std::io::ErrorKind::InvalidData)?;


    let fixed = gtk4::Fixed::new();

    let character = Rc::new(RefCell::new(gtk4::Image::from_file("./res/pikachu_sprites/stationary0.png")));
    let x_position = Rc::new(RefCell::new(100.0));
    let stationary = Rc::new(RefCell::new(true));

    character.borrow().set_pixel_size(CHARACTER_SIZE);
    fixed.put(character.borrow().deref(), *x_position.borrow(), 0.0);

    window.set_child(Some(&fixed));
    window.set_default_size(CHARACTER_SIZE, CHARACTER_SIZE);
    window.set_resizable(false);

    let character_clone = Rc::clone(&character);
    let stationary_clone = Rc::clone(&stationary);
    let x_position_clone = Rc::clone(&x_position);
    let mut frame = 0;

    // animate character
    timeout_add_local(Duration::from_millis(1000 / FPS as u64), move || {
        if *(stationary_clone.borrow().deref()) {
            let file_path = format!("./res/pikachu_sprites/stationary{}.png", frame);
            character_clone.borrow().set_from_file(Some(&file_path));
            frame = (frame + 1) % 4;
        } else {
            let file_path = format!("./res/pikachu_sprites/run_right{}.png", frame);
            character_clone.borrow().set_from_file(Some(&file_path));
            frame = (frame + 1) % 3;
        }

        ControlFlow::from(true)
    });

    let character_clone = Rc::clone(&character);
    let stationary_clone = Rc::clone(&stationary);
    // move character
    timeout_add_local(Duration::from_millis(1000 / MOVEMENT_SPEED as u64), move || {
        if !*(stationary_clone.borrow().deref()) && character_clone.borrow().file().is_some() && character_clone.borrow().file().unwrap().contains("run") {
            // update position
            let mut value = x_position_clone.borrow_mut();
            *value = (*value + 10.0) % screen_width as f64;
            // move along screen
            fixed.move_(character_clone.borrow().deref(), *value, 0.0);
        }
        ControlFlow::from(true)
    });

    // change state of character (stationary/running)
    let gesture = GestureClick::new();
    gesture.connect_pressed(move |_gesture: &GestureClick, _n_press: i32, _x: f64, _y: f64| {
        let mut value = stationary.borrow_mut();
        *value = !*value;
    });

    character.borrow().add_controller(gesture);

    Ok(())
}

fn screen_width(window: &ApplicationWindow) -> Option<i32> {
    let display = Display::default()?;

    let monitor = display.monitor_at_surface(&window.surface()?)?;
    Some(monitor.geometry().width())
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(r#"* {
        background-color: transparent;
    }"#);

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    )
}

fn main() {
    let application = gtk4::Application::new(Some("chickensoftware.hqnnqh.goose"), Default::default());

    application.connect_startup(|_| load_css());

    application.connect_activate(|app| {
        let result = activate(app);

        if result.is_err() {
            eprintln!("An error occurred: {:?}", result.err());
        }
    });

    application.run();
}
