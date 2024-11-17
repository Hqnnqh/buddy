use std::env;

use crate::config::{cli::Cli, Config};
use crate::error::BuddyError;
use clap::Parser;
use regex::Regex;

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

/// Parse [Cli] and config arguments. Returns [Config] structure and sprites path. [BuddyError] is returned in case of failirue (invalid config, invalid sprites path).
///
/// Note: sprites path in config structure remains None.
pub(crate) fn run() -> Result<(Config, String), BuddyError> {
    let cli = Cli::parse();

    // load specific config file or default path.
    let mut config: Config = match cli.config_path {
        Some(config_path) => confy::load_path(config_path),
        None => confy::load("buddy", Option::from("config")),
    }
    .map_err(BuddyError::from)?;

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
    let sprites_path = config
        .sprites_path
        .take()
        .and_then(|path| expand_env(path.replace("~", "$HOME")))
        .ok_or(BuddyError::NoSprites)?;

    Ok((config, sprites_path))
}

/// Expand environment variables in paths of config file.
fn expand_env(input: String) -> Option<String> {
    Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").ok().map(|re| {
        re.replace_all(&input, |caps: &regex::Captures| {
            env::var(&caps[1]).unwrap_or_else(|_| format!("${}", &caps[1]))
        })
        .to_string()
    })
}
