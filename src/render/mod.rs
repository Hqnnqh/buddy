use std::cell::Cell;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use gio::prelude::{ApplicationExt, ApplicationExtManual};
use glib::{timeout_add_local, ControlFlow};
use gtk4::prelude::{GtkWindowExt, WidgetExt};
use gtk4::{ApplicationWindow, GestureClick};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use helpers::load_css;
use helpers::screen_resolution;
use helpers::update_input_region;
use rand::Rng;
use sprite::preload_images;
use state::State;

use crate::config::Config;
use crate::error::BuddyError;

mod helpers;
mod sprite;
mod state;

/// Prepare and render character.
pub(crate) fn render_character(config: Config, sprites_path: String) {
    let app_id = format!("hqnnqh.buddy.instance{}", std::process::id());

    let application = gtk4::Application::new(Some(app_id.as_str()), Default::default());

    application.connect_startup(|_| load_css());

    let sprites_path = Rc::new(sprites_path);

    application.connect_activate(move |app| {
        let result = activate(app, &config, &sprites_path);

        if let Err(err) = result {
            eprintln!("An error occurred: {}", err);
            std::process::exit(1);
        }
    });
    application.run_with_args::<&str>(&[]);
}

/// Active GTK app. May fail and return [BuddyError].
fn activate(
    application: &gtk4::Application,
    config: &Config,
    sprites_path: &Rc<String>,
) -> Result<(), BuddyError> {
    // used to handle signal to reload sprites
    let reload_sprites = Arc::new(AtomicBool::new(false));

    signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&reload_sprites))
        .map_err(BuddyError::from)?;

    signal_hook::flag::register(signal_hook::consts::SIGUSR2, Arc::clone(&reload_sprites))
        .map_err(BuddyError::from)?;

    let Config {
        character_size,
        fps,
        movement_speed,
        onclick_event_chance,
        x,
        y,
        left,
        flip_horizontal,
        flip_vertical,
        debug,
        signal_frequency,
        automatic_reload,
        ..
    } = *config;

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

    let (screen_width, screen_height) =
        screen_resolution(&window).ok_or(BuddyError::NoScreenResolution)?;

    // check for valid starting coordinates
    if !debug
        && ((x + character_size as i32) >= screen_width
            || x < 0
            || (y + character_size as i32) >= screen_height
            || y < 0)
    {
        return Err(BuddyError::CoordinatesOutOfBounds(
            x,
            y,
            screen_width,
            screen_height,
            character_size,
        ));
    }

    let character_size = character_size as i32;

    let sprites = Rc::new(RefCell::new(preload_images(
        Path::new(sprites_path.as_str()),
        flip_horizontal,
        flip_vertical,
    )?));

    // start with idle sprites
    let character = Rc::new(gtk4::Image::from_paintable(Some(
        &sprites.as_ref().borrow().0[0],
    )));

    let state = Rc::new(Cell::new(State::Idle));
    character.set_pixel_size(character_size);

    // default position
    character.set_margin_start(x);
    character.set_margin_bottom(y);

    window.set_child(Some(&*character));
    window.set_default_size(character_size, character_size);
    window.set_resizable(false);

    let sprites_clone = Rc::clone(&sprites);
    let sprites_path_clone = Rc::clone(sprites_path);

    timeout_add_local(
        Duration::from_millis(1000 / signal_frequency as u64),
        move || {
            if automatic_reload || reload_sprites.swap(false, Ordering::Relaxed) {
                match preload_images(
                    Path::new(sprites_path_clone.as_str()),
                    flip_horizontal,
                    flip_vertical,
                ) {
                    Ok(sprites) => *sprites_clone.borrow_mut() = sprites,
                    Err(err) => println!("Warning: Could not update sprites: {}", err),
                }
            }
            ControlFlow::from(true)
        },
    );

    let character_clone = Rc::clone(&character);
    let state_clone = Rc::clone(&state);

    let mut frame = 0;

    // animate character
    timeout_add_local(Duration::from_millis(1000 / fps as u64), move || {
        match (*state_clone).get() {
            State::Idle => {
                frame = (frame + 1) % sprites.as_ref().borrow().0.len();
                character_clone.set_paintable(Some(&sprites.as_ref().borrow().0[frame]));
            }
            State::InitiatingClick => {
                frame = 0;
                state_clone.set(State::Click);
            }
            State::Click => {
                if frame == sprites.as_ref().borrow().2.len() {
                    state_clone.set(State::Idle);
                    frame = 0;
                } else {
                    character_clone.set_paintable(Some(&sprites.as_ref().borrow().2[frame]));

                    frame += 1;
                }
            }
            // Running
            State::Running | State::InitiatingRun => {
                frame = (frame + 1) % sprites.as_ref().borrow().1.len();

                character_clone.set_paintable(Some(&sprites.as_ref().borrow().1[frame]));

                if state_clone.get() == State::InitiatingRun {
                    state_clone.set(State::Running)
                }
            }
        }
        ControlFlow::from(true)
    });

    let character_clone = Rc::clone(&character);
    let state_clone = Rc::clone(&state);

    // move character
    timeout_add_local(
        Duration::from_millis(1000 / movement_speed as u64),
        move || {
            if state_clone.get() == State::Running {
                // update position
                let value = if left {
                    let new_position = character_clone.margin_start() - 10;
                    if new_position <= -(character_size * 2) {
                        (screen_width + 10) as f64
                    } else {
                        new_position as f64
                    }
                } else {
                    (character_clone.margin_start() as f64 + 10.0) % (screen_width as f64 + 10.0)
                };
                // move along screen
                character_clone.set_margin_start(value as i32);
                update_input_region(&window, character_size, value as i32, 0);
            }
            ControlFlow::from(true)
        },
    );

    // change state of character (idle/initiating run)
    let gesture = GestureClick::new();

    gesture.connect_pressed(
        move |_gesture: &GestureClick, _n_press: i32, _x: f64, _y: f64| {
            if state.get() != State::Click && state.get() != State::InitiatingClick {
                // initiate click event
                if state.get() == State::Idle
                    && (rand::thread_rng().gen_range(0..100) + 1) as u8 <= onclick_event_chance
                {
                    state.set(State::InitiatingClick);
                } else {
                    state.set(!state.get());
                }
            }
        },
    );

    character.add_controller(gesture);
    Ok(())
}
