use serde_derive::{Deserialize, Serialize};

pub(crate) mod cli;
pub(crate) mod default;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    // can safely be casted as both i32 and u32
    pub(crate) character_size: u16,
    pub(crate) fps: u32,
    pub(crate) movement_speed: u32,
    pub(crate) onclick_event_chance: u8,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) sprites_path: Option<String>,
    pub(crate) left: bool,
    pub(crate) flip_horizontal: bool,
    pub(crate) flip_vertical: bool,
    pub(crate) debug: bool,
    pub(crate) signal_frequency: u32,
    pub(crate) automatic_reload: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            character_size: default::CHARACTER_SIZE,
            fps: default::FPS,
            movement_speed: default::MOVEMENT_SPEED,
            onclick_event_chance: default::ON_CLICK_CHANCE,
            x: default::X,
            y: default::Y,
            left: default::RUN_LEFT,
            flip_horizontal: default::FLIP_HORIZONTAL,
            flip_vertical: default::FLIP_VERTICAL,
            debug: default::DEBUG,
            signal_frequency: default::SIGNAL_FREQUENCY,
            automatic_reload: default::AUTOMATIC_RELOAD,
            sprites_path: None,
        }
    }
}
