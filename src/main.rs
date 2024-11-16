use std::env;

use clap::Parser;
use cli::Cli;

mod cli;
mod render;
mod state;

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
            left: cli.left,
            flip_horizontal: cli.flip_horizontal,
            flip_vertical: cli.flip_vertical,
            debug: cli.debug,
            signal_frequency: cli.signal_frequency,
            automatic_reload: cli.automatic_reload,
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
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) sprites_path: String,
    pub(crate) left: bool,
    pub(crate) flip_horizontal: bool,
    pub(crate) flip_vertical: bool,
    pub(crate) debug: bool,
    pub(crate) signal_frequency: u32,
    pub(crate) automatic_reload: bool,
}
