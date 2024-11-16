use std::cell::Cell;
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use std::vec::Vec;

use gdk4::cairo::{RectangleInt, Region};
use gdk4::gdk_pixbuf::Pixbuf;
use gdk4::prelude::{DisplayExt, MonitorExt};
use gdk4::{Display, Texture};
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use glib::{timeout_add_local, ControlFlow};
use gtk4::prelude::{GtkWindowExt, NativeExt, WidgetExt};
use gtk4::{ApplicationWindow, CssProvider, GestureClick};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use rand::Rng;

use crate::state::State;

fn activate(application: &gtk4::Application, config: &Rc<Config>) -> Result<(), glib::Error> {
    // used to handle signal to reload sprites
    let reload_sprites = Arc::new(AtomicBool::new(false));

    signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&reload_sprites))
        .map_err(|err| {
            glib::Error::new(
                glib::FileError::Io,
                format!("Cannot subscribe to signal handler SUGUSR1: {}", err).as_str(),
            )
        })?;

    signal_hook::flag::register(signal_hook::consts::SIGUSR2, Arc::clone(&reload_sprites))
        .map_err(|err| {
            glib::Error::new(
                glib::FileError::Io,
                format!("Cannot subscribe to signal handler SIGUSR2: {}", err).as_str(),
            )
        })?;

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
    } = *config.as_ref();

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

    if let Some((screen_width, screen_height)) = screen_resolution(&window) {
        // check for valid starting coordinates
        if !debug
            && ((x + character_size as i32) >= screen_width
                || x < 0
                || (y + character_size as i32) >= screen_height
                || y < 0)
        {
            return Err(glib::Error::new(
                glib::FileError::Failed,
                format!("Starting coordinates out of bounds: x: {}px, y: {}px for screen width: {}px, screen height: {}px, character size: {}px", x, y, screen_width, screen_height, character_size).as_str(),
            ));
        }

        let character_size = character_size as i32;

        let sprites = Rc::new(RefCell::new(preload_images(
            Path::new(config.sprites_path.as_str()),
            flip_horizontal,
            flip_vertical,
        )?));

        // start with idle sprites
        let character = Rc::new(gtk4::Image::from_paintable(Some(&sprites.borrow().0[0])));
        let state = Rc::new(Cell::new(State::Idle));
        character.set_pixel_size(character_size);

        // default position
        character.set_margin_start(x);
        character.set_margin_bottom(y);

        window.set_child(Some(&*character));
        window.set_default_size(character_size, character_size);
        window.set_resizable(false);

        let config_clone = Rc::clone(config);
        let sprites_clone = Rc::clone(&sprites);

        timeout_add_local(
            Duration::from_millis(1000 / signal_frequency as u64),
            move || {
                if automatic_reload || reload_sprites.swap(false, Ordering::Relaxed) {
                    match preload_images(
                        Path::new(config_clone.sprites_path.as_str()),
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
                    frame = (frame + 1) % sprites.borrow().0.len();
                    character_clone.set_paintable(Some(&sprites.borrow().0[frame]));
                }
                State::InitiatingClick => {
                    frame = 0;
                    state_clone.set(State::Click);
                }
                State::Click => {
                    if frame == sprites.borrow().2.len() {
                        state_clone.set(State::Idle);
                        frame = 0;
                    } else {
                        character_clone.set_paintable(Some(&sprites.borrow().2[frame]));

                        frame += 1;
                    }
                }
                // Running
                State::Running | State::InitiatingRun => {
                    frame = (frame + 1) % sprites.borrow().1.len();

                    character_clone.set_paintable(Some(&sprites.borrow().1[frame]));

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
                        (character_clone.margin_start() as f64 + 10.0)
                            % (screen_width as f64 + 10.0)
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
    } else {
        Err(glib::Error::new(
            glib::FileError::Failed,
            "Cannot get display resolution!",
        ))
    }
}

type Sprites = (Vec<Texture>, Vec<Texture>, Vec<Texture>);

fn preload_images(
    sprites_path: &Path,
    flip_horizontal: bool,
    flip_vertical: bool,
) -> Result<Sprites, glib::Error> {
    // Preload images for better performance
    let mut idle = Vec::default();
    let mut running = Vec::default();
    let mut click = Vec::default();

    let animations = ["idle", "run", "click"];
    for animation in animations {
        let animation_path = sprites_path.join(animation);
        if let Ok(entry) = std::fs::read_dir(&animation_path) {
            let mut files = entry
                .filter_map(|file| file.ok())
                .filter(|file| {
                    file.metadata()
                        .ok()
                        .map_or(false, |metadata| metadata.is_file())
                })
                .map(|file| file.file_name())
                .collect::<Vec<OsString>>();
            files.sort();

            let textures: Result<Vec<Texture>, glib::Error> = files
                .into_iter()
                .filter_map(|file_name| {
                    file_name
                        .to_str()
                        .map(|file_name| animation_path.join(file_name))
                })
                .map(|file_path| {
                    let mut pixbuf = Pixbuf::from_file(file_path)?;

                    if flip_horizontal {
                        pixbuf = pixbuf.flip(true).ok_or(glib::Error::new(
                            glib::FileError::Failed,
                            "Could not flip sprites horizontally",
                        ))?
                    }

                    if flip_vertical {
                        pixbuf = pixbuf.flip(false).ok_or(glib::Error::new(
                            glib::FileError::Failed,
                            "Could not flip sprites vertically",
                        ))?
                    }

                    Ok(Texture::for_pixbuf(&pixbuf))
                })
                .collect();
            match animation {
                "idle" => idle = textures?,
                "run" => running = textures?,
                "click" => click = textures?,
                _ => {
                    return Err(glib::Error::new(
                        glib::FileError::Failed,
                        "Unexpected animation type",
                    ))
                }
            }
        }
    }

    if idle.is_empty() || running.is_empty() || click.is_empty() {
        Err(glib::Error::new(
            glib::FileError::Failed,
            "Sprites cannot be found!",
        ))
    } else {
        Ok((idle, running, click))
    }
}

use gdk4::prelude::SurfaceExt;

use crate::Config;
fn update_input_region(window: &ApplicationWindow, character_size: i32, x: i32, y: i32) {
    let region = Region::create_rectangle(&RectangleInt::new(x, y, character_size, character_size));
    window.surface().unwrap().set_input_region(&region);
}
/// Returns the screen resolution (width, height). May fail and return None.
fn screen_resolution(window: &ApplicationWindow) -> Option<(i32, i32)> {
    let display = Display::default()?;

    let monitor = display.monitor_at_surface(&window.surface()?)?;
    Some((monitor.geometry().width(), monitor.geometry().height()))
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

pub fn render_character(config: Config) {
    let app_id = format!("hqnnqh.buddy.instance{}", std::process::id());

    let application = gtk4::Application::new(Some(app_id.as_str()), Default::default());

    application.connect_startup(|_| load_css());

    let config = Rc::new(config);
    application.connect_activate(move |app| {
        let result = activate(app, &config);

        if let Err(err) = result {
            eprintln!("An error occurred: {:?}", err.message());
            std::process::exit(1);
        }
    });
    application.run_with_args::<&str>(&[]);
}
