use std::cell::RefCell;
use std::ops::{Deref, Not};
use std::rc::Rc;
use std::time::Duration;
use std::vec::Vec;

use gdk4::{Display, Texture};
use gdk4::gdk_pixbuf::Pixbuf;
use gdk4::prelude::{DisplayExt, MonitorExt};
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use glib::{ControlFlow, timeout_add_local};
use gtk4::{ApplicationWindow, CssProvider, GestureClick};
use gtk4::prelude::{FixedExt, GtkWindowExt, NativeExt, WidgetExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

const CHARACTER_SIZE: i32 = 75;
const FPS: u32 = 4;
const MOVEMENT_SPEED: u32 = 20;
const SPRITE_PATH: &str = "./res/pikachu_sprites/";

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Stationary,
    // special state for initiating run to ensure proper timing
    InitiatingRun,
    Running,
}

impl Not for State {
    type Output = State;

    fn not(self) -> Self::Output {
        match self {
            State::Running | State::InitiatingRun => State::Stationary,
            State::Stationary => State::InitiatingRun,
        }
    }
}

fn activate(application: &gtk4::Application) -> Result<(), glib::Error> {
    let window = ApplicationWindow::new(application);

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

    if let Some(screen_width) = screen_width(&window) {
        // Preload images for better performance
        let mut stationary_sprites = Vec::default();
        let mut running_sprites = Vec::default();

        for i in 0..4 {
            stationary_sprites.push(Texture::for_pixbuf(&Pixbuf::from_file(format!(
                "{}stationary{}.png",
                SPRITE_PATH, i
            ))?));
        }
        for i in 0..3 {
            running_sprites.push((
                Texture::for_pixbuf(&Pixbuf::from_file(format!(
                    "{}run_left{}.png",
                    SPRITE_PATH, i
                ))?),
                Texture::for_pixbuf(&Pixbuf::from_file(format!(
                    "{}run_right{}.png",
                    SPRITE_PATH, i
                ))?),
            ));
        }

        let fixed = gtk4::Fixed::new();

        let character = Rc::new(RefCell::new(gtk4::Image::from_paintable(Some(
            &stationary_sprites[0],
        ))));
        let x_position = Rc::new(RefCell::new(100.0));
        let state = Rc::new(RefCell::new(State::Stationary));

        character.borrow().set_pixel_size(CHARACTER_SIZE);
        fixed.put(character.borrow().deref(), *x_position.borrow(), 0.0);

        window.set_child(Some(&fixed));
        window.set_default_size(CHARACTER_SIZE, CHARACTER_SIZE);
        window.set_resizable(false);

        let character_clone = Rc::clone(&character);
        let state_clone = Rc::clone(&state);
        let x_position_clone = Rc::clone(&x_position);
        let mut frame = 0;

        // animate character
        timeout_add_local(Duration::from_millis(1000 / FPS as u64), move || {
            if *(state_clone.borrow().deref()) == State::Stationary {
                frame = (frame + 1) % stationary_sprites.len();
                character_clone
                    .borrow()
                    .set_paintable(Some(&stationary_sprites[frame]));
            } else {
                frame = (frame + 1) % running_sprites.len();

                character_clone
                    .borrow()
                    .set_paintable(Some(&running_sprites[frame].1));

                if *(state_clone.borrow().deref()) == State::InitiatingRun {
                    *(state_clone.borrow_mut()) = State::Running;
                }
            }

            ControlFlow::from(true)
        });

        let character_clone = Rc::clone(&character);
        let state_clone = Rc::clone(&state);

        // move character
        timeout_add_local(
            Duration::from_millis(1000 / MOVEMENT_SPEED as u64),
            move || {
                if *(state_clone.borrow().deref()) == State::Running {
                    // update position
                    let mut value = x_position_clone.borrow_mut();
                    *value = (*value + 10.0) % screen_width as f64;
                    // move along screen
                    fixed.move_(character_clone.borrow().deref(), *value, 0.0);
                }
                ControlFlow::from(true)
            },
        );

        // change state of character (stationary/initiating run)
        let gesture = GestureClick::new();
        gesture.connect_pressed(
            move |_gesture: &GestureClick, _n_press: i32, _x: f64, _y: f64| {
                let mut value = state.borrow_mut();
                *value = !*value;
            },
        );

        character.borrow().add_controller(gesture);
        Ok(())
    } else {
        Err(glib::Error::new(
            glib::FileError::Failed,
            "Cannot get display resolution!",
        ))
    }
}

fn screen_width(window: &ApplicationWindow) -> Option<i32> {
    let display = Display::default()?;

    let monitor = display.monitor_at_surface(&window.surface()?)?;
    Some(monitor.geometry().width())
}

fn load_css() {
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

fn main() {
    let application =
        gtk4::Application::new(Some("chickensoftware.hqnnqh.goose"), Default::default());

    application.connect_startup(|_| load_css());

    application.connect_activate(|app| {
        let result = activate(app);

        if result.is_err() {
            eprintln!("An error occurred: {:?}", result.err());
        }
    });

    application.run();
}
