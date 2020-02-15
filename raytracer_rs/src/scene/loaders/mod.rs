pub mod boxloader;
pub mod colladaloader;

use super::Scene;
use std::{error, fmt};

pub trait SceneLoader {
    fn from_str(input: &str) -> Result<Scene, SceneLoadError>;
    fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Scene, SceneLoadError>;
    fn load() -> Result<Scene, SceneLoadError>;
}

#[derive(Debug)]
pub enum SceneLoadError {
    ColladaLoader(colladaloader::ColladaError),
    BoxLoader(String),
    Io(std::io::Error),
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
            SceneLoadError::Io(e) => write!(f, "{}", e.to_string()),
            SceneLoadError::BoxLoader(s) => write!(f, "{}", s),
        }
    }
}
impl error::Error for SceneLoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            SceneLoadError::ColladaLoader(e) => Some(e),
            SceneLoadError::Io(e) => Some(e),
            SceneLoadError::BoxLoader(_) => None,
        }
    }
}
