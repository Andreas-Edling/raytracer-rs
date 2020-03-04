
use super::color;
use image;

use std::{io, fmt, error};

pub struct Texture {
    width: usize,
    height: usize,
    data: Vec<color::RGB>,
}

impl Texture {
    fn new(width: usize, height: usize, data: Vec<color::RGB>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn get_texel(&self, u: f32, v: f32) -> &color::RGB {

        // nearest neighbour "filtering"
        let x = (u * self.width as f32) as usize;
        let y = (v * self.height as f32) as usize;

        &self.data[y*self.width + x]
    }
}

pub trait TextureLoader {
    fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Texture, TextureLoadError>;
}

impl TextureLoader for Texture {
    fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Texture, TextureLoadError> {
        let image = image::open(path)?.to_rgb();
        let (w,h) = image.dimensions();
        let data = image.pixels().map(|pix| {
            color::RGB::new(
                pix[0] as f32 / 256.0,
                pix[1] as f32 / 256.0,
                pix[2] as f32 / 256.0,
            )
        }).collect();
        Ok(Texture::new(w as usize,h as usize, data))
    }
}

// -- Error Handling ----------------------------------------------------------

#[derive(Debug)]
pub enum TextureLoadError {
    ImageError(image::error::ImageError),
    Io(io::Error),
}

impl From<io::Error> for TextureLoadError {
    fn from(e: io::Error) -> Self {
        TextureLoadError::Io(e)
    }
}

impl From<image::error::ImageError> for TextureLoadError {
    fn from(e: image::error::ImageError) -> Self {
        TextureLoadError::ImageError(e)
    }
}

impl fmt::Display for TextureLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextureLoadError::Io(e) => write!(f, "{}", e.to_string()),
            TextureLoadError::ImageError(e) => write!(f, "{}", e.to_string()),
        }
    }
}

impl error::Error for TextureLoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            TextureLoadError::Io(e) => Some(e),
            TextureLoadError::ImageError(e) => Some(e),
        }
    }
}