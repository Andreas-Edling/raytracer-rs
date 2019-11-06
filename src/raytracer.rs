use crate::vecmath::{cross, dot, Vec3};

type Pos = Vec3;
type Vertex = Vec3;

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
    pub fn test_triangle() -> Self {
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

    pub fn test_box() -> Self {
        let mut vertices = Vec::with_capacity(36);
        const LEFT: f32 = 0.3;
        const RIGHT: f32 = 0.7;
        const UP: f32 = 0.7;
        const DOWN: f32 = 0.3;
        const NEAR: f32 = 1.3;
        const FAR: f32 = 1.7;

        // near / far
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, NEAR));
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, NEAR));
        vertices.push(Vertex::new(LEFT, UP, NEAR));
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
        vertices.push(Vertex::new(RIGHT, DOWN, FAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));
        vertices.push(Vertex::new(RIGHT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));
        vertices.push(Vertex::new(RIGHT, UP, NEAR));

        // up / down
        vertices.push(Vertex::new(LEFT, UP, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));
        vertices.push(Vertex::new(LEFT, UP, NEAR));
        vertices.push(Vertex::new(RIGHT, UP, FAR));
        vertices.push(Vertex::new(LEFT, UP, FAR));
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, DOWN, FAR));
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(RIGHT, DOWN, FAR));
        vertices.push(Vertex::new(LEFT, DOWN, FAR));

        let lights = vec![Light::new(
            Pos::new(0.0, 0.0, 0.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
        )];

        Scene { vertices, lights }
    }
}

#[derive(Clone)]
pub struct Ray {
    pos: Pos,
    dir: Vec3,
}
impl Ray {
    fn new<P: Into<Pos>, V: Into<Vec3>>(pos: P, dir: V) -> Self {
        Ray {
            pos: pos.into(),
            dir: dir.into(),
        }
    }
}

fn intersect(ray: &Ray, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Option<f32> {
    // Möller-Trumbore algo

    let v0v1 = v1 - v0;
    let v0v2 = v2 - v0;
    let pvec = cross(&ray.dir, &v0v2);
    let det = dot(&v0v1, &pvec);

    // ray and triangle are parallel if det is close to 0
    if det.abs() < std::f32::EPSILON {
        // switch to "if det < std::f32::EPSILON { return None };" for backface culling
        return None;
    }
    let inv_det = 1.0 / det;

    let tvec = &ray.pos - v0;
    let u = dot(&tvec, &pvec) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let qvec = cross(&tvec, &v0v1);
    let v = dot(&ray.dir, &qvec) * inv_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    // u,v are coords in tri, return if needed
    let t = dot(&v0v2, &qvec) * inv_det;
    Some(t)
}

pub struct RayTracer {
    scene: Scene,
    width: usize,
    height: usize,
    pub camera: camera::Camera,
}

impl RayTracer {
    pub fn new(width: usize, height: usize, scene: Scene) -> Self {
        RayTracer {
            scene,
            width,
            height,
            camera: camera::Camera::ortho(width, height),
        }
    }

    #[rustfmt::skip]
    pub fn trace_frame(&mut self) -> Vec<u32> {
        let rays = self.camera.get_rays();
        let mut frame = Vec::with_capacity(self.width*self.height);

        for ray in rays {
            let mut closest_hit = None;
            for tri_vertices in self.scene.vertices.chunks(3) {
                match (closest_hit, intersect(ray, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2])) {
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(x)) => if x > 0.0 {closest_hit = Some(x)},
                    (Some(c), Some(x)) => if x > 0.0 && x < c {closest_hit = Some(x)},
                }
            }

            let color = match closest_hit {
                Some(_) => 0xFF_FF_FF_FFu32,
                None => 0x00_00_00_00u32,
            };
            frame.push(color);
        }
        frame
    }
}


mod camera {
    use super::{Pos, Ray, Vec3};
    use crate::vecmath::*;

    pub struct Camera {
        rays: Vec<Ray>,
        transformed_rays: Vec<Ray>,
        x_angle_radians: f32,
        y_angle_radians: f32,
        pos: Vec3,
    }

    impl Camera {
        pub fn ortho(width: usize, height: usize) -> Self {
            let mut rays = Vec::<Ray>::with_capacity(width * height);

            for y in 0..height {
                for x in 0..width {
                    rays.push(Ray::new(
                        Pos::new(x as f32 / width as f32, y as f32 / height as f32, 0.0),
                        Vec3::new(0.0, 0.0, 1.0),
                    ));
                }
            }

            let transformed_rays = rays.clone();

            Camera {
                rays,
                transformed_rays,
                x_angle_radians: 0.0,
                y_angle_radians: 0.0,
                pos: Vec3::new(0.0,0.0,0.0),
            }
        }

        pub fn set_x_angle(&mut self, radians: f32) {
            self.x_angle_radians = radians;
        }

        pub fn set_y_angle(&mut self, radians: f32) {
            self.y_angle_radians = radians;
        }

        pub fn set_pos(&mut self, pos: Vec3) {
            self.pos = pos;
        }

        pub fn add_x_angle(&mut self, radians: f32) {
            self.x_angle_radians += radians;
        }

        pub fn add_y_angle(&mut self, radians: f32) {
            self.y_angle_radians += radians;
        }

        pub fn move_rel(&mut self, x: f32, y: f32, z: f32) {
            self.pos.x += x;
            self.pos.y += y;
            self.pos.z += z;
        }

        pub fn get_rays<'a>(&'a mut self) -> &'a [Ray] {
            let matrix = Matrix::rot_x(self.x_angle_radians);
            let matrix = matrix * Matrix::rot_y(self.y_angle_radians);
            let matrix = matrix * Matrix::translate(&self.pos);

            for (i, ray) in self.rays.iter().enumerate() {
                let pos = &matrix * Vec4::from_vec3(&ray.pos);
                let mut dir = &matrix * Vec4::from_vec3(&ray.dir);
                dir.w = 1.0;
                self.transformed_rays[i] = Ray::new(pos, dir);
            }
            &self.transformed_rays
        }
    }
}
