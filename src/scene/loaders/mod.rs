pub mod boxloader;

use super::Scene;

pub trait SceneLoader {
    fn from_str(s: &str) -> Result<Scene, &str>;
    fn load() -> Result<Scene, &'static str>;
}
