struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}
impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex { x, y, z }
    }
}

type Pos = Vertex;
type Vec3 = Vertex;

struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
impl Color {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

struct Light {
    pos: Pos,
    color: Color,
}
impl Light {
    fn new(pos: Pos, color: Color) -> Self {
        Light { pos, color }
    }
}

pub struct Scene {
    vertices: Vec<Vertex>,
    lights: Vec<Light>,
}
impl Scene {
    pub fn test_scene() -> Self {
        let mut vertices = Vec::with_capacity(3);
        vertices.push(Vertex::new(0.0, 0.0, 1.0));
        vertices.push(Vertex::new(1.0, 0.0, 1.0));
        vertices.push(Vertex::new(0.5, 0.8, 1.0));

        let lights = vec![Light::new(
            Pos::new(0.0, 0.0, 0.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
        )];

        Scene { vertices, lights }
    }
}

pub struct Ray {
    pos: Pos,
    dir: Vec3,
}
impl Ray {
    fn new(pos: Pos, dir: Vec3) -> Self {
        Ray { pos, dir }
    }
}

fn intersect(ray: &Ray, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Option<f32> {
    Some(1.0)
}

pub struct RayTracer {
    scene: Scene,
    width: usize,
    height: usize,
}

mod camera {
    use super::{Pos, Ray, Vec3};

    pub fn naive_ortho(width: usize, height: usize) -> Vec<Ray> {
        let mut rays = Vec::<Ray>::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                rays.push(Ray::new(
                    Pos::new(x as f32, y as f32, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ));
            }
        }
        rays
    }
}

impl RayTracer {
    pub fn new(width: usize, height: usize, scene: Scene) -> Self {
        RayTracer {
            scene,
            width,
            height,
        }
    }

    pub fn trace_frame(&self) -> Vec<u32> {
        let rays = camera::naive_ortho(self.width, self.height);
        let mut frame = Vec::with_capacity(self.width*self.height);

        for ray in &rays {
            let mut closest_hit = None;
            for tri_vertices in self.scene.vertices.chunks(3) {
                match (closest_hit, intersect(ray, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2])) {
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(x)) => if x > 0.0 {closest_hit = Some(x)},
                    (Some(c), Some(x)) => if x > 0.0 && x<c {closest_hit = Some(x)},
                }
            }

            let color = match closest_hit {
                Some(_) => 0xFF_FF_FF_FFu32,
                None => 0x00_00_00_FFu32,
            };
            frame.push(color);
        }
        frame
    }
}
