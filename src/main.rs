mod config;
mod error;
mod parse;
mod render;

fn main() {
    match parse::run() {
        Ok((config, sprites_path)) => render::render_character(config, sprites_path),
        Err(err) => eprintln!("{}", err),
    }
}
