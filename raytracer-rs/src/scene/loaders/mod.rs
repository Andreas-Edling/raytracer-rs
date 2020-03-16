pub mod colladaloader;

use super::Scene;
use std::{error, fmt, path};

pub trait SceneLoader {
    fn from_str(
        doc: &str,
        data_dir: Option<&path::Path>,
        width: usize,
        height: usize,
    ) -> Result<Scene, SceneLoadError>;
    fn from_file<P: AsRef<path::Path>>(
        path: P,
        width: usize,
        height: usize,
    ) -> Result<Scene, SceneLoadError>;
}

#[derive(Debug)]
pub enum SceneLoadError {
    ColladaLoader(colladaloader::ColladaError),
    TextureLoader(super::texture::TextureLoadError),
    Io(std::io::Error),
}

impl From<super::texture::TextureLoadError> for SceneLoadError {
    fn from(e: super::texture::TextureLoadError) -> Self {
        SceneLoadError::TextureLoader(e)
    }
}

impl From<colladaloader::ColladaError> for SceneLoadError {
    fn from(e: colladaloader::ColladaError) -> Self {
        SceneLoadError::ColladaLoader(e)
    }
}

impl From<std::io::Error> for SceneLoadError {
    fn from(e: std::io::Error) -> Self {
        SceneLoadError::Io(e)
    }
}

impl fmt::Display for SceneLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SceneLoadError::ColladaLoader(e) => write!(f, "{}", e.to_string()),
            SceneLoadError::TextureLoader(e) => write!(f, "{}", e.to_string()),
            SceneLoadError::Io(e) => write!(f, "{}", e.to_string()),
        }
    }
}
impl error::Error for SceneLoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            SceneLoadError::ColladaLoader(e) => Some(e),
            SceneLoadError::TextureLoader(e) => Some(e),
            SceneLoadError::Io(e) => Some(e),
        }
    }
}
