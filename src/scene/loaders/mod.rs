pub mod boxloader;
pub mod colladaloader;

use super::Scene;

#[derive(Debug, Eq, PartialEq)]
pub enum SceneLoadError {
    ColladaLoader(colladaloader::ColladaError),
    BoxLoader(String),
}
pub trait SceneLoader {
    fn from_str(input: &str) -> Result<Scene, SceneLoadError>;
    fn load() -> Result<Scene, SceneLoadError>;
}
