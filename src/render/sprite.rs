use std::vec::Vec;
use std::{ffi::OsString, path::Path};

use gdk4::{gdk_pixbuf::Pixbuf, Texture};

use crate::error::BuddyError;

/// Animation sprites
pub(super) type Sprites = (Vec<Texture>, Vec<Texture>, Vec<Texture>);

/// (Pre-)load the images for better preformance. May fail and return [BuddyError].
pub(super) fn preload_images(
    sprites_path: &Path,
    flip_horizontal: bool,
    flip_vertical: bool,
) -> Result<Sprites, BuddyError> {
    // Preload images for better performance
    let mut idle = Vec::default();
    let mut running = Vec::default();
    let mut click = Vec::default();

    let animations = ["idle", "run", "click"];
    for animation in animations {
        let animation_path = sprites_path.join(animation);
        if let Ok(entry) = std::fs::read_dir(&animation_path) {
            let mut files = entry
                .filter_map(|file| file.ok())
                .filter(|file| {
                    file.metadata()
                        .ok()
                        .map_or(false, |metadata| metadata.is_file())
                })
                .map(|file| file.file_name())
                .collect::<Vec<OsString>>();
            files.sort();

            let textures: Result<Vec<Texture>, BuddyError> = files
                .into_iter()
                .filter_map(|file_name| {
                    file_name
                        .to_str()
                        .map(|file_name| animation_path.join(file_name))
                })
                .map(|file_path| {
                    let mut pixbuf = Pixbuf::from_file(file_path).map_err(BuddyError::from)?;

                    if flip_horizontal {
                        pixbuf = pixbuf.flip(true).ok_or(BuddyError::FlipFailed(true))?;
                    }

                    if flip_vertical {
                        pixbuf = pixbuf.flip(false).ok_or(BuddyError::FlipFailed(false))?;
                    }

                    Ok(Texture::for_pixbuf(&pixbuf))
                })
                .collect();

            match animation {
                "idle" => idle = textures?,
                "run" => running = textures?,
                "click" => click = textures?,
                animation => return Err(BuddyError::UnexpectedAnimation(animation.to_string())),
            }
        }
    }

    if idle.is_empty() || running.is_empty() || click.is_empty() {
        Err(BuddyError::SpritesCannotBeFound(
            sprites_path.to_string_lossy().to_string(),
        ))
    } else {
        Ok((idle, running, click))
    }
}
