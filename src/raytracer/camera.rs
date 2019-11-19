
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
            pos: Vec3::new(-0.5, -0.5, -1.0),
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
                self.transformed_rays[i] = Ray::new(pos.into(), dir.into());
            }
            self.orientation_changed = false;
        }
        &self.transformed_rays
    }
}