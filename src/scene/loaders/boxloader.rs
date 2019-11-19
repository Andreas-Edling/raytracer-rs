
pub use super::SceneLoader;

use crate::scene::{
    Scene, 
    Vertex,
    Pos,
    color::RGB,
    Light,
};

pub struct BoxLoader {}

impl SceneLoader for BoxLoader {
    fn from_str(_s: &str) -> Result<Scene, &str> {
        Err("not implemented for BoxLoader")
    }

    fn load() -> Result<Scene, &'static str> {
        let mut vertices = Vec::with_capacity(36);
        const LEFT: f32 = -0.5;
        const RIGHT: f32 = 0.5;
        const UP: f32 = 0.5;
        const DOWN: f32 = -0.5;
        const NEAR: f32 = -0.5;
        const FAR: f32 = 0.5;

        // near / far
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, NEAR));
        vertices.push(Vertex::new(RIGHT, DOWN, NEAR));
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(LEFT, UP, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, NEAR));
        vertices.push(Vertex::new(LEFT, DOWN, FAR));
        vertices.push(Vertex::new(RIGHT, DOWN, FAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));
        vertices.push(Vertex::new(LEFT, DOWN, FAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));
        vertices.push(Vertex::new(LEFT, UP, FAR));

        // left / right
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(LEFT, DOWN, FAR));
        vertices.push(Vertex::new(LEFT, UP, FAR));
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(LEFT, UP, FAR));
        vertices.push(Vertex::new(LEFT, UP, NEAR));
        vertices.push(Vertex::new(RIGHT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));
        vertices.push(Vertex::new(RIGHT, DOWN, FAR));
        vertices.push(Vertex::new(RIGHT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));

        // up / down
        vertices.push(Vertex::new(LEFT, UP, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));
        vertices.push(Vertex::new(RIGHT, UP, NEAR));
        vertices.push(Vertex::new(LEFT, UP, NEAR));
        vertices.push(Vertex::new(LEFT, UP, FAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, DOWN, FAR));
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, DOWN, FAR));
        vertices.push(Vertex::new(LEFT, DOWN, FAR));

        let transformed_vertices = vertices.clone();

        let lights = vec![Light::new(
            Pos::new(RIGHT*3.0, UP*2.0, NEAR*2.0),
            RGB::new(1.0, 1.0, 1.0),
        )];

        Ok( Scene { vertices, lights, transformed_vertices } )
    }
}