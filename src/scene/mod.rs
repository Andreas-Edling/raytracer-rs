
pub mod boxloader;
pub mod color;

use crate::vecmath::*;
use color::{RGB};

pub type Pos = Vec3;
pub type Vertex = Vec3;


pub struct Light {
    pub pos: Pos,
    pub color: RGB,
}
impl Light {
    fn new(pos: Pos, color: RGB) -> Self {
        Light { pos, color }
    }
}


pub struct Scene {
    vertices: Vec<Vertex>,
    pub lights: Vec<Light>,

    pub transformed_vertices: Vec<Vertex>,
}


pub trait SceneLoader {
    fn from_str(s: &str) -> Result<Scene, &str>;
    fn load() -> Result<Scene, &'static str>;
}


impl Scene {
    pub fn apply_transform(&mut self, mat: &Matrix) {
        for (vtx, transformed_vtx) in self.vertices.iter().zip(self.transformed_vertices.iter_mut()) {
                *transformed_vtx = Vec3::from(mat * Vec4::from_vec3(vtx));
        }
    }
}
