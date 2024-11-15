use clap::Parser;
use clap_num::number_range;

#[derive(Parser)]
#[command(name = "Buddy")]
#[command(author = "Hannah F. <github: Hqnnqh>")]
#[command(version = "1.0")]
#[command(about = r#"Your new best buddy when using your computer :)!"#, long_about = None)]
pub(crate) struct Cli {
    #[clap(
        short = 's',
        long,
        value_name = "PATH",
        help = "Initial path to directory with animation sprites. Defaults to environment variable 'BUDDY_SPRITES_PATH'. Directory must contains subdirectories for each event type."
    )]
    pub(crate) sprites_path: Option<String>,
    #[clap(
        default_value_t = 75,
        short,
        long,
        value_name = "SIZE",
        help = "Size of character in pixels (should match animation sprites)."
    )]
    pub(crate) character_size: u16,

    #[clap(
        default_value_t = 4,
        short,
        long,
        value_name = "AMOUNT",
        help = "Frames per second to animate character."
    )]
    pub(crate) fps: u32,

    #[clap(
        default_value_t = 20,
        short,
        long,
        value_name = "AMOUNT",
        help = "How often the character's position is updated per second."
    )]
    pub(crate) movement_speed: u32,

    #[clap(
        default_value_t = 1,
        short = 'S',
        long,
        value_name = "AMOUNT",
        help = "How often to check for signals per second. Or how often to reload sprites if automatic reload is enabled."
    )]
    pub(crate) signal_frequency: u32,

    #[clap(
        default_value_t = false,
        short = 'r',
        long,
        value_name = "AUTOMATIC-RELOAD",
        help = "Enables the automatic reload of sprites, the frequency should be specific using -S."
    )]
    pub(crate) automatic_reload: bool,

    #[clap(
        default_value_t = 15,
        short,
        long,
        value_name = "PERCENT",
        value_parser = less_than_101,
        help = "Chance of on-click event occurring."
    )]
    pub(crate) onclick_event_chance: u8,
    #[clap(
        default_value_t = 100,
        short,
        long,
        value_name = "X-START",
        help = "Starting position of buddy on x-axis."
    )]
    pub(crate) x: u32,
    #[clap(
        default_value_t = 0,
        short,
        long,
        value_name = "Y-START",
        help = "Starting position of buddy on y-axis."
    )]
    pub(crate) y: u32,
    #[clap(
        default_value_t = false,
        short,
        long,
        value_name = "RUN-LEFT",
        help = "Make buddy move to the left instead of the default: right"
    )]
    pub(crate) left: bool,
    #[clap(
        default_value_t = false,
        short = 'H',
        long,
        value_name = "FLIP-HORIZONTAL",
        help = "Used to flip the horizontal direction of sprites."
    )]
    pub(crate) flip_horizontal: bool,
    #[clap(
        default_value_t = false,
        short = 'v',
        long,
        value_name = "FLIP-VERTICAL",
        help = "Used to flip the vertical direction of sprites."
    )]
    pub(crate) flip_vertical: bool,
    #[clap(
        default_value_t = false,
        short,
        long,
        value_name = "DEBUG-MODE",
        help = "Used to disable out of bounds checks."
    )]
    pub(crate) debug: bool,
}

fn less_than_101(s: &str) -> Result<u8, String> {
    number_range(s, 0, 100)
}
