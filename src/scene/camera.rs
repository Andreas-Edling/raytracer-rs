use rand::Rng;

use crate::vecmath::{Matrix, Ray, Vec3, Vec4};

#[derive(Debug, Clone)]
pub struct Camera {
    rays: Vec<Ray>,
    transformed_rays: Vec<Ray>,
    x_angle_radians: f32,
    y_angle_radians: f32,
    pos: Vec3,
    orientation_changed: bool,
    width: usize,
    height: usize,
    half_fov: f32,
}

impl Camera {
    pub fn new(width: usize, height: usize, pos: &Vec3, fov_deg: f32) -> Self {
        let matrix = Matrix::translate(pos);
        Self::from_orientation_matrix(width, height, &matrix, fov_deg)
    }

    // Note - only rotation and position is expected/used in matrix, no perspective!
    pub fn from_orientation_matrix(
        width: usize,
        height: usize,
        orientation_matrix: &Matrix,
        fov_deg: f32,
    ) -> Self {
        let rotation_matrix = {
            let mut rotation_matrix = *orientation_matrix;
            rotation_matrix[3] = 0.0;
            rotation_matrix[7] = 0.0;
            rotation_matrix[11] = 0.0;

            rotation_matrix[12] = 0.0;
            rotation_matrix[13] = 0.0;
            rotation_matrix[14] = 0.0;
            rotation_matrix[15] = 1.0;
            rotation_matrix
        };

        let mut rays = Vec::<Ray>::with_capacity(width * height);

        let fov = fov_deg * std::f32::consts::PI / 180.0;
        let half_fov = 0.5 * fov;
        let max_x = 1.0 * half_fov.tan();
        let max_y = 1.0 * half_fov.tan();

        for y in 0..height {
            let dir_y = -max_y + 2.0 * max_y * (y as f32 / height as f32);
            for x in 0..width {
                let dir_x = -max_x + 2.0 * max_x * (x as f32 / width as f32);

                let pos = Vec3::new(x as f32 / width as f32, 1.0 - y as f32 / height as f32, 0.0);
                let dir = Vec3::new(dir_x, -dir_y, 1.0);
                let pos = orientation_matrix * Vec4::from_vec3(&pos);
                let mut dir = rotation_matrix * Vec4::from_vec3(&dir);
                dir.w = 1.0;
                rays.push(Ray::new(pos.into(), dir.into()));
            }
        }
        let transformed_rays = rays.clone();

        Camera {
            rays,
            transformed_rays,
            x_angle_radians: 0.0,
            y_angle_radians: 0.0,
            pos: Vec3::new(0.0, 0.0, 0.0), //pos: Vec3::new(-0.5, -0.5, -10.0),
            orientation_changed: true,
            width,
            height,
            half_fov,
        }
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

    #[allow(dead_code)]
    pub fn get_rays(&mut self) -> &[Ray] {
        if self.orientation_changed {
            let matrix = Matrix::rot_x(self.x_angle_radians);
            let matrix = matrix * Matrix::rot_y(self.y_angle_radians);
            let pos_matrix = matrix * Matrix::translate(&self.pos);

            for (i, ray) in self.rays.iter().enumerate() {
                let pos = pos_matrix * Vec4::from_vec3(&ray.pos);
                let mut dir = matrix * Vec4::from_vec3(&ray.dir);
                dir.w = 1.0;
                self.transformed_rays[i] = Ray::new(pos.into(), dir.into());
            }
            self.orientation_changed = false;
        }
        &self.transformed_rays
    }

    pub fn get_jittered_rays(&mut self) -> &[Ray] {
        let mut rng = rand::thread_rng();
        let matrix = Matrix::rot_x(self.x_angle_radians);
        let matrix = matrix * Matrix::rot_y(self.y_angle_radians);
        let pos_matrix = matrix * Matrix::translate(&self.pos);

        for (i, ray) in self.rays.iter().enumerate() {
            let pos = pos_matrix * Vec4::from_vec3(&ray.pos);
            let mut dir = ray.dir.clone();
            dir.x += rng.gen_range(0.0, 1.0) / self.width as f32;
            dir.y += rng.gen_range(0.0, 1.0) / self.height as f32;
            let mut dir = matrix * Vec4::from_vec3(&dir);
            dir.w = 1.0;
            self.transformed_rays[i] = Ray::new(pos.into(), dir.into());
        }
        &self.transformed_rays
    }
}
