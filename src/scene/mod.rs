pub mod camera;
pub mod color;
pub mod loaders;

pub use crate::vecmath::*;
use camera::Camera;
use color::RGB;

pub type Vertex = Vec3;

#[derive(Debug, Clone)]
pub struct Light {
    pub pos: Vec3,
    pub color: RGB,
}

impl Light {
    fn new(pos: Vec3, color: RGB) -> Self {
        Light { pos, color }
    }
}

pub struct Scene {
    pub geometries: Vec<Geometry>,
    pub lights: Vec<Light>,
    pub cameras: Vec<Camera>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn apply_transform(&mut self, mat: &Matrix) {
        for geom in self.geometries.iter_mut() {
            for (vtx, transformed_vtx) in geom
                .vertices
                .iter()
                .zip(geom.transformed_vertices.iter_mut())
            {
                *transformed_vtx = Vec3::from(mat * Vec4::from_vec3(vtx));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub transformed_vertices: Vec<Vertex>,
    pub material: Material,
}
impl Geometry {
    pub fn new(vertices: Vec<Vertex>, material: Material) -> Self {
        let transformed_vertices = vertices.clone();
        Geometry {
            vertices,
            transformed_vertices,
            material,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Material {
    pub diffuse: RGB,
    pub emissive: RGB,
    pub specular: Option<f32>,
    pub index_of_refraction: f32,
}
