pub mod boxloader;
pub mod colladaloader;

use super::Scene;

pub trait SceneLoader {
    fn from_str(input: &str) -> Result<Scene, &str>;
    fn load() -> Result<Scene, &'static str>;
}
