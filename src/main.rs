use std::env;

use clap::Parser;
use clap_num::number_range;

mod render;

#[derive(Parser)]
#[command(name = "Buddy")]
#[command(author = "Hannah F. <github: Hqnnqh>")]
#[command(version = "1.0")]
#[command(about = r#"Your new best buddy when using your computer :)!"#, long_about = None)]
struct Cli {
    #[clap(
        short = 's',
        long,
        value_name = "PATH",
        help = "Initial path to directory with animation sprites. Defaults to environment variable 'BUDDY_SPRITES_PATH'. Directory must contains subdirectories for each event type."
    )]
    sprites_path: Option<String>,
    #[clap(
        default_value_t = 75,
        short,
        long,
        value_name = "SIZE",
        help = "Size of character in pixels (should match animation sprites)."
    )]
    character_size: u16,

    #[clap(
        default_value_t = 4,
        short,
        long,
        value_name = "SECONDS",
        help = "Frames per second to animate character."
    )]
    fps: u32,

    #[clap(
        default_value_t = 20,
        short,
        long,
        value_name = "SECONDS",
        help = "Movement speed of character."
    )]
    movement_speed: u32,
    #[clap(
        default_value_t = 15,
        short,
        long,
        value_name = "PERCENT",
        value_parser = less_than_101,
        help = "Chance of on-click event occurring."
    )]
    onclick_event_chance: u8,
    #[clap(
        default_value_t = 100,
        short,
        long,
        value_name = "X-START",
        help = "Starting position of buddy on x-axis."
    )]
    x: u32,
    #[clap(
        default_value_t = 0,
        short,
        long,
        value_name = "Y-START",
        help = "Starting position of buddy on y-axis."
    )]
    y: u32,
}

fn less_than_101(s: &str) -> Result<u8, String> {
    number_range(s, 0, 100)
}

fn main() {
    let cli = Cli::parse();

    if let Some(sprites_path) = cli
        .sprites_path
        .or_else(|| env::var("BUDDY_SPRITES_PATH").ok())
    {
        println!("Initializing Buddy with character size: {}px, fps: {}s, movement speed: {}fps, on-click event chance: {}%, sprites path: {}, x-start: {}, y-start: {}.", cli.character_size, cli.fps, cli.movement_speed, cli.onclick_event_chance, &sprites_path, cli.x, cli.y);
        render::render_character(Config {
            character_size: cli.character_size,
            fps: cli.fps,
            movement_speed: cli.movement_speed,
            onclick_event_chance: cli.onclick_event_chance,
            sprites_path,
            x: cli.x,
            y: cli.y,
        });
    } else {
        eprintln!("Path to directory of animation sprites cannot be found! Try buddy -h for more information!");
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Config {
    // can safely be casted as both i32 and u32
    pub(crate) character_size: u16,
    pub(crate) fps: u32,
    pub(crate) movement_speed: u32,
    pub(crate) onclick_event_chance: u8,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) sprites_path: String,
}
