use crate::vecmath::{cross, dot, Vec3, Vec4, Matrix};

type Pos = Vec3;
type Vertex = Vec3;

struct RGB {
    r: f32,
    g: f32,
    b: f32,
}
impl RGB {
    fn new(r: f32, g: f32, b: f32) -> Self {
        RGB { r, g, b }
    }
}
impl std::ops::AddAssign for RGB {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

#[rustfmt::skip] impl std::ops::Mul<f32> for &RGB { type Output = RGB; fn mul(self, other: f32) -> RGB { RGB::new(self.r * other, self.g * other, self.b * other) }}
#[rustfmt::skip] impl std::ops::Mul<f32> for  RGB { type Output = RGB; fn mul(self, other: f32) -> RGB { RGB::new(self.r * other, self.g * other, self.b * other) }}
#[rustfmt::skip] impl std::ops::Mul< RGB> for f32 { type Output = RGB; fn mul(self, other: RGB) -> RGB { RGB::new(self * other.r, self * other.g, self * other.b) }}
#[rustfmt::skip] impl std::ops::Mul<&RGB> for f32 { type Output = RGB; fn mul(self, other: &RGB) -> RGB { RGB::new(self * other.r, self * other.g, self * other.b) }}

struct RGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
impl RGBA {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        RGBA { r, g, b, a }
    }

    fn from_rgb(rgb: RGB, a: f32) -> Self {
        RGBA{ r: rgb.r, g: rgb.g, b: rgb.b, a}
    }

    fn to_u32(&self) -> u32 {
        let r = (self.r.min(1.0).max(0.0)*255.0) as u8;
        let g = (self.g.min(1.0).max(0.0)*255.0) as u8;
        let b = (self.b.min(1.0).max(0.0)*255.0) as u8;
        let a = (self.a.min(1.0).max(0.0)*255.0) as u8;
        let res = r as u32 | (g as u32) << 8 | (b as u32) << 16 | (a as u32) << 24;
        res
    }
}



pub struct Light {
    pos: Pos,
    color: RGB,
}
impl Light {
    fn new(pos: Pos, color: RGB) -> Self {
        Light { pos, color }
    }
}

pub struct Scene {
    vertices: Vec<Vertex>,
    lights: Vec<Light>,

    transformed_vertices: Vec<Vertex>,
}
impl Scene {
    pub fn test_triangle() -> Self {
        let mut vertices = Vec::with_capacity(3);
        vertices.push(Vertex::new(0.0, 0.0, 1.0));
        vertices.push(Vertex::new(1.0, 0.0, 1.0));
        vertices.push(Vertex::new(0.5, 0.8, 1.0));

        let transformed_vertices = vertices.clone();

        let lights = vec![Light::new(
            Pos::new(0.0, 0.0, 0.0),
            RGB::new(1.0, 1.0, 1.0),
        )];

        Scene { vertices, lights, transformed_vertices }
    }

    pub fn test_box() -> Self {
        let mut vertices = Vec::with_capacity(36);
        const LEFT: f32 = -0.5;
        const RIGHT: f32 = 0.5;
        const UP: f32 = 0.5;
        const DOWN: f32 = -0.5;
        const NEAR: f32 = -0.5;
        const FAR: f32 = 0.5;

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
        vertices.push(Vertex::new(RIGHT, DOWN, FAR));
        vertices.push(Vertex::new(RIGHT, DOWN, NEAR));
        vertices.push(Vertex::new(LEFT, DOWN, NEAR));
        vertices.push(Vertex::new(LEFT, DOWN, FAR));
        vertices.push(Vertex::new(RIGHT, DOWN, FAR));

        let transformed_vertices = vertices.clone();

        let lights = vec![Light::new(
            Pos::new(RIGHT*3.0, UP*2.0, NEAR*2.0),
            RGB::new(1.0, 1.0, 1.0),
        )];


        Scene { vertices, lights, transformed_vertices }
    }

    pub fn apply_transform(&mut self, mat: &Matrix) {
        for (vtx, transformed_vtx) in self.vertices.iter().zip(self.transformed_vertices.iter_mut()) {
                *transformed_vtx = Vec3::from(mat * Vec4::from_vec3(vtx));

        }
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
    pub scene: Scene,
    width: usize,
    height: usize,
    pub camera: camera::Camera,
}

struct Hit {
    distance: f32,
    vertex_index: usize,
}
impl Hit {
    fn new(distance: f32, vertex_index: usize) -> Self {
        Hit{ distance, vertex_index}
    }
}

impl RayTracer {
    pub fn new(width: usize, height: usize, scene: Scene) -> Self {
        RayTracer {
            scene,
            width,
            height,
            camera: camera::Camera::new(width, height),
        }
    }

    #[rustfmt::skip]
    pub fn trace_frame(&mut self) -> Vec<u32> {
        let rays = self.camera.get_rays();
        println!("middle ray- p {} {} {}  d {} {} {}",
            rays[320 +240*640].pos.x,
            rays[320 +240*640].pos.y,
            rays[320 +240*640].pos.z,
            rays[320 +240*640].dir.x,
            rays[320 +240*640].dir.y,
            rays[320 +240*640].dir.z,
        );
        let mut frame = Vec::with_capacity(self.width*self.height);

        let mut hits = false;
        let mut printdist = -666.0;
        for ray in rays {
            let mut closest_hit = None;
            for (i, tri_vertices) in self.scene.transformed_vertices.chunks(3).enumerate() {
                match (&closest_hit, intersect(ray, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2])) {
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(dist)) => {
                        if dist > 0.0 { 
                            closest_hit = Some(Hit::new(dist, i));
                            printdist = dist;
                        }
                    },
                    (Some(hit), Some(dist)) => {
                        if dist > 0.0 && dist < hit.distance {
                            closest_hit = Some(Hit::new(dist, i));
                            printdist = dist;
                        }
                    },
                }
            }

            let coloru32 = match closest_hit {
                Some(ref hit) => {
                    let rgb = RayTracer::shade(ray, hit, &self.scene.lights, &self.scene.transformed_vertices);
                    let c = RGBA::from_rgb(rgb, 1.0).to_u32();
                    hits = true;
                    c
                },
                None => 0x00_00_00_00u32,
            };
            frame.push(coloru32);
        }
        println!("printdist {}", printdist);
        println!("hits {}",hits);
        frame
    }

    fn shade(ray: &Ray, hit: &Hit, lights: &[Light], vertices: &[Vertex]) -> RGB {

        let mut accum_color = RGB::new(1.0, 0.0, 0.0);
        let hit_point = &ray.pos + hit.distance * &ray.dir ;
        'outer: for light in lights {

                 let ray_to_light = Ray::new(hit_point.clone(), &light.pos - &hit_point);
                // is light blocked by geometry?
                // for tri_vertices in vertices.chunks(3) {
                //     if let Some(t) = intersect(&ray_to_light, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2]) {
                //         if t > 0.0001 && t < 1.0 {
                //             continue 'outer;
                //         }
                //     }
                // }

                let normal = cross( 
                    &(&vertices[hit.vertex_index+1] - &vertices[hit.vertex_index]), 
                    &(&vertices[hit.vertex_index+2] - &vertices[hit.vertex_index]));
                let normal = normal.normalized();

                let val = dot(&normal, &ray_to_light.dir.normalized());
            
                if val>0.0 {
                    accum_color += val * &light.color;
                }
        }
        accum_color
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
        orientation_changed: bool,
    }

    impl Camera {
        pub fn new(width: usize, height: usize) -> Self {
            let mut rays = Vec::<Ray>::with_capacity(width * height);

            let fov = 60.0f32 *3.1415/180.0;
            let half_fov = 0.5*fov;
            let max_x = 1.0 * half_fov.tan();
            let max_y = 1.0 * half_fov.tan();

            for y in 0..height {
                let dir_y = -max_y + 2.0*max_y*(y as f32 / height as f32);
                for x in 0..width {
                    let dir_x = -max_x + 2.0*max_x*(x as f32 / width as f32);
                    rays.push(Ray::new(
                        Pos::new(x as f32 / width as f32, y as f32 / height as f32, 0.0),
                        Vec3::new(dir_x, dir_y, 1.0),
                    ));
                }
            }

            let transformed_rays = rays.clone();

            Camera {
                rays,
                transformed_rays,
                x_angle_radians: 0.0,
                y_angle_radians: 0.0,
                pos: Vec3::new(0.0, 0.0, 0.0),
                orientation_changed: true,
            }
        }

        pub fn set_x_angle(&mut self, radians: f32) {
            self.orientation_changed |= self.x_angle_radians != radians;
            self.x_angle_radians = radians;

        }

        pub fn set_y_angle(&mut self, radians: f32) {
            self.orientation_changed |= self.y_angle_radians != radians;
            self.y_angle_radians = radians;
        }

        pub fn set_pos(&mut self, pos: Vec3) {
            self.orientation_changed |= self.pos != pos;
            self.pos = pos;
        }

        pub fn add_x_angle(&mut self, radians: f32) {
            self.orientation_changed |= radians != 0.0;
            self.x_angle_radians += radians;
        }

        pub fn add_y_angle(&mut self, radians: f32) {
            self.orientation_changed |= radians != 0.0;
            self.y_angle_radians += radians;
        }

        pub fn move_rel(&mut self, x: f32, y: f32, z: f32) {
            self.orientation_changed |= x != 0.0 || y != 0.0 || z != 0.0;
            self.pos.x += x;
            self.pos.y += y;
            self.pos.z += z;
        }

        pub fn get_rays<'a>(&'a mut self) -> &'a [Ray] {
            if self.orientation_changed {
                let matrix = Matrix::rot_x(self.x_angle_radians);
                let matrix = matrix * Matrix::rot_y(self.y_angle_radians);
                let pos_matrix = &matrix * Matrix::translate(&self.pos);

                for (i, ray) in self.rays.iter().enumerate() {
                    let pos = &pos_matrix * Vec4::from_vec3(&ray.pos);
                    let mut dir = &matrix * Vec4::from_vec3(&ray.dir);
                    dir.w = 1.0;
                    self.transformed_rays[i] = Ray::new(pos, dir);
                }
                self.orientation_changed = false;
            }
            &self.transformed_rays
        }
    }
}
