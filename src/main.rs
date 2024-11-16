use std::env;

use clap::Parser;
use config::{cli::Cli, Config};
use confy::ConfyError;
use regex::Regex;

mod config;
mod render;
mod state;

/// Parse cli args and match against config file
macro_rules! parse_args {
    ($config:expr, $cli:expr, $($field:ident),*) => {{
        $(
            if let Some(value) = $cli.$field {
                $config.$field = value.into();
            }
        )*
    }};
}
fn main() {
    let cli = Cli::parse();

    // load specific config file or default path.
    let config: Result<Config, ConfyError> = match cli.config_path {
        Some(config_path) => confy::load_path(config_path),
        None => confy::load("buddy", Option::from("config")),
    };

    let mut config = match config {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Configuration Error: {}", err);
            return;
        }
    };

    parse_args!(
        config,
        cli,
        sprites_path,
        character_size,
        fps,
        movement_speed,
        signal_frequency,
        automatic_reload,
        onclick_event_chance,
        x,
        y,
        left,
        flip_horizontal,
        flip_vertical,
        debug
    );

    // check for existing sprites path
    let sprites_path = match config
        .sprites_path
        .take()
        .and_then(|path| expand_env(path.replace("~", "$HOME")))
    {
        Some(sprites_path) => sprites_path,
        None => {
            eprintln!("No sprites path provided!");
            return;
        }
    };

    render::render_character(config, sprites_path);
}

fn expand_env(input: String) -> Option<String> {
    Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").ok().map(|re| {
        re.replace_all(&input, |caps: &regex::Captures| {
            env::var(&caps[1]).unwrap_or_else(|_| format!("${}", &caps[1]))
        })
        .to_string()
    })
}
