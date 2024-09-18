use std::cell::Cell;
use std::cmp::PartialEq;
use std::ffi::OsString;
use std::ops::Not;
use std::rc::Rc;
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Stationary,

    // special state for initiating run to ensure proper timing
    InitiatingRun,
    Running,

    InitiatingExplosion,
    Explosion,
}

impl Not for State {
    type Output = State;

    fn not(self) -> Self::Output {
        match self {
            State::Running
            | State::InitiatingRun
            | State::InitiatingExplosion
            | State::Explosion => State::Stationary,
            State::Stationary => State::InitiatingRun,
        }
    }
}

fn activate(application: &gtk4::Application, config: &Config) -> Result<(), glib::Error> {
    let Config {
        character_size,
        fps,
        movement_speed,
        onclick_event_chance,
        x,
        y,
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

    if let Some((screen_width, screen_height)) = screen_resolution(&window) {
        // check for valid starting coordinates
        if (x + character_size as u32) >= screen_width as u32
            || (y + character_size as u32) >= screen_height as u32
        {
            return Err(glib::Error::new(
                glib::FileError::Failed,
                format!("Starting coordinates out of bounds: x: {}px, y: {}px for screen width: {}px, screen height: {}px, character size: {}px", x, y, screen_width, screen_height, character_size).as_str(),
            ));
        }

        let character_size = character_size as i32;
        let x = x as i32;

        let (stationary_sprites, running_sprites, explosion_sprites) =
            preload_images(config.sprites_path.as_str())?;

        if stationary_sprites.is_empty()
            || running_sprites.is_empty()
            || explosion_sprites.is_empty()
        {
            return Err(glib::Error::new(
                glib::FileError::Failed,
                "Sprites cannot be found!",
            ));
        }

        let character = Rc::new(gtk4::Image::from_paintable(Some(&stationary_sprites[0])));
        let state = Rc::new(Cell::new(State::Stationary));

        character.set_pixel_size(character_size);

        // default position
        character.set_margin_start(x);
        character.set_margin_bottom(y as i32);

        window.set_child(Some(&*character));
        window.set_default_size(character_size, character_size);
        window.set_resizable(false);

        let character_clone = Rc::clone(&character);
        let state_clone = Rc::clone(&state);
        let mut frame = 0;

        // animate character
        timeout_add_local(Duration::from_millis(1000 / fps as u64), move || {
            match (*state_clone).get() {
                State::Stationary => {
                    frame = (frame + 1) % stationary_sprites.len();
                    character_clone.set_paintable(Some(&stationary_sprites[frame]));
                }
                State::InitiatingExplosion => {
                    frame = 0;
                    state_clone.set(State::Explosion);
                }
                State::Explosion => {
                    if frame == explosion_sprites.len() {
                        state_clone.set(State::Stationary);
                        frame = 0;
                    } else {
                        character_clone.set_paintable(Some(&explosion_sprites[frame]));

                        frame += 1;
                    }
                }
                // Running
                State::Running | State::InitiatingRun => {
                    frame = (frame + 1) % running_sprites.len();

                    character_clone.set_paintable(Some(&running_sprites[frame]));

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
                    let mut value = character_clone.margin_start() as f64;
                    value = (value + 10.0) % (screen_width + 10) as f64;
                    // move along screen
                    character_clone.set_margin_start(value as i32);
                    update_input_region(&window, character_size, value as i32, 0);
                }
                ControlFlow::from(true)
            },
        );

        // change state of character (stationary/initiating run)
        let gesture = GestureClick::new();

        gesture.connect_pressed(
            move |_gesture: &GestureClick, _n_press: i32, _x: f64, _y: f64| {
                if state.get() != State::Explosion && state.get() != State::InitiatingExplosion {
                    // initiate explosion event
                    if state.get() == State::Stationary
                        && (rand::thread_rng().gen_range(0..100) + 1) as u8 <= onclick_event_chance
                    {
                        state.set(State::InitiatingExplosion);
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

fn preload_images(sprites_path: &str) -> Result<Sprites, glib::Error> {
    // Preload images for better performance
    let mut stationary = Vec::default();
    let mut running = Vec::default();
    let mut explosion = Vec::default();

    let animations = ["stationary", "run", "explode"];
    for animation in animations {
        if let Ok(entry) = std::fs::read_dir(format!("{sprites_path}{animation}")) {
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
                        .map(|file_name| format!("{sprites_path}{animation}/{}", file_name))
                })
                .map(|file_path| {
                    let pixbuf = Pixbuf::from_file(file_path)?;
                    Ok(Texture::for_pixbuf(&pixbuf))
                })
                .collect();

            match animation {
                "stationary" => stationary = textures?,
                "run" => running = textures?,
                "explode" => explosion = textures?,
                _ => {
                    return Err(glib::Error::new(
                        glib::FileError::Failed,
                        "Unexpected animation type",
                    ))
                }
            }
        }
    }
    Ok((stationary, running, explosion))
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
    let application = gtk4::Application::new(Some("hqnnqh.buddy"), Default::default());

    application.connect_startup(|_| load_css());

    application.connect_activate(move |app| {
        let result = activate(app, &config);

        if result.is_err() {
            eprintln!("An error occurred: {:?}", result.err().unwrap().message());
            std::process::exit(1);
        }
    });
    application.run_with_args::<&str>(&[]);
}
