use confy::ConfyError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum BuddyError {
    #[error("Configuration Failed: {0}")]
    InvalidConfig(#[from] ConfyError),
    #[error("No sprites path specidied")]
    NoSprites,
    #[error("Graphical Failure: {0}")]
    Glib(#[from] gio::glib::Error),
    #[error("Signal Subscription Failed: {0}")]
    SignalSubscriptionFailed(#[from] std::io::Error),
    #[error("Coordinates out of bounds: x: {0}px, y: {1}px for screen width: {2}px, screen height: {3}px, character size: {4}px - Use debug flag to disable bounds-checking")]
    CoordinatesOutOfBounds(i32, i32, i32, i32, u16),
    #[error("Unable to get screen resolution!")]
    NoScreenResolution,
    #[error("Could not flip buddy on horizontal axis: {0}(/vertical axis)")]
    FlipFailed(bool),
    #[error("Unexpected animation type in sprites folder: {0}")]
    UnexpectedAnimation(String),
    #[error("Sprites cannot be found at path: {0}")]
    SpritesCannotBeFound(String),
}
