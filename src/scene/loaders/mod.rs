pub mod boxloader;
pub mod colladaloader;

use super::Scene;

pub trait SceneLoader {
    fn from_str(input: &str) -> Result<Scene, String>;
    fn load() -> Result<Scene, String>;
}
