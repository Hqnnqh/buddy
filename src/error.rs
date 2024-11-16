use std::{error, fmt};

use confy::ConfyError;

#[derive(Debug)]
pub(crate) enum BuddyError {
    InvalidConfig(ConfyError),
    NoSprites,
    Glib(glib::Error),
    SignalSubscriptionFailed(std::io::Error),
    CoordinatesOutOfBounds(i32, i32, i32, i32, u16),
    NoScreenResolution,
    FlipFailed(bool),
    UnexpectedAnimation(String),
    SpritesCannotBeFound(String),
}

impl fmt::Display for BuddyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            BuddyError::InvalidConfig(confy) => {
                format!("Configuration Failed: {}", confy)
            }
            BuddyError::NoSprites => "No sprites path specidied".to_string(),
            BuddyError::Glib(glib) => format!("Graphical Failirue: {}", glib),
            BuddyError::SignalSubscriptionFailed(err) => {
                format!("Signal Subscription Failed: {}", err)
            }
            BuddyError::CoordinatesOutOfBounds(
                x,
                y,
                screen_width,
                screen_height,
                character_size,
            ) => {
                format!("Coordinates out of bounds: x: {}px, y: {}px for screen width: {}px, screen height: {}px, character size: {}px - Use debug flag to disable bounds-checking", x, y, screen_width, screen_height, character_size)
            }
            BuddyError::NoScreenResolution => "Unable to get screen resolution!".to_string(),
            BuddyError::FlipFailed(horizontal) => {
                format!(
                    "Could not flip buddy {}",
                    if *horizontal {
                        "horizontally"
                    } else {
                        "vertically"
                    }
                )
            }
            BuddyError::UnexpectedAnimation(animation) => {
                format!("Unexpected animation type in sprites folder: {}", animation)
            }
            BuddyError::SpritesCannotBeFound(path) => {
                format!("Sprites cannot be found at path: {}", path)
            }
        };

        write!(f, "Buddy Error: {}.", message)
    }
}

impl error::Error for BuddyError {}

impl From<ConfyError> for BuddyError {
    fn from(value: ConfyError) -> Self {
        Self::InvalidConfig(value)
    }
}

impl From<glib::Error> for BuddyError {
    fn from(value: glib::Error) -> Self {
        Self::Glib(value)
    }
}

impl From<std::io::Error> for BuddyError {
    fn from(value: std::io::Error) -> Self {
        Self::SignalSubscriptionFailed(value)
    }
}
