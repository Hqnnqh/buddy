use std::env;

use clap::Parser;
use clap_num::number_range;

mod render;

#[derive(Parser)]
#[command(name = "ChickenBuddy")]
#[command(author = "Hannah F. <github: Hqnnqh>")]
#[command(version = "1.0")]
#[command(about = r#"Your new best buddy when using your computer :)!"#, long_about = None)]
struct Cli {
    #[clap(
        short = 's',
        long,
        value_name = "PATH",
        help = "Initial path to directory with animation sprites. Defaults to environment variable 'CHICKEN_BUDDY_SPRITES_PATH'. Directory must contains subdirectories for each event type."
    )]
    sprites_path: Option<String>,
    #[clap(
        default_value_t = 75,
        short,
        long,
        value_name = "SIZE",
        help = "Size of character in pixels (should match animation sprites)."
    )]
    character_size: i32,

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
}

fn less_than_101(s: &str) -> Result<u8, String> {
    number_range(s, 0, 100)
}

fn main() {
    let cli = Cli::parse();

    if let Some(sprites_path) = cli.sprites_path.or_else(|| env::var("CHICKEN_BUDDY_SPRITES_PATH").ok()) {
        println!("Initializing Chicken Buddy with character size: {}px, fps: {}s, movement speed: {}fps, on-click event chance: {}%, sprites path: {}.", cli.character_size, cli.fps, cli.movement_speed, cli.onclick_event_chance, &sprites_path);
        render::render_character(cli.character_size, cli.fps, cli.movement_speed, cli.onclick_event_chance, sprites_path);
    } else {
        eprintln!("Path to directory of animation sprites cannot be found! Try chickenbuddy -h for more information!");
    }
}