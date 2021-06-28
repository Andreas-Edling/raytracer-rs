use rand::Rng;

use crate::vecmath::{Matrix, Ray, Vec3, Vec4};

#[derive(Debug, Clone)]
pub struct Camera {
    x_angle_radians: f32,
    y_angle_radians: f32,
    pos: Vec3,
    width: usize,
    height: usize,
    half_fov: f32,
    base_orientation_matrix: Matrix,
    base_rotation_matrix: Matrix,
    orientation_matrix: Matrix,
    rotation_matrix: Matrix,
    max_x: f32,
    max_y: f32,
}

impl Camera {
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

        let fov = fov_deg * std::f32::consts::PI / 180.0;
        let half_fov = 0.5 * fov;
        let max_x = 1.0 * half_fov.tan();
        let max_y = 1.0 * half_fov.tan();

        let mut cam = Camera {
            x_angle_radians: 0.0,
            y_angle_radians: 0.0,
            pos: Vec3::new(0.0, 0.0, 0.0),
            width,
            height,
            half_fov,
            base_orientation_matrix: *orientation_matrix,
            base_rotation_matrix: rotation_matrix,
            orientation_matrix: Matrix::ident(),
            rotation_matrix: Matrix::ident(),
            max_x,
            max_y,
        };
        cam.update_matrices();
        cam
    }

    pub fn add_x_angle(&mut self, radians: f32) {
        self.x_angle_radians += radians;
        self.update_matrices();
    }

    pub fn add_y_angle(&mut self, radians: f32) {
        self.y_angle_radians += radians;
        self.update_matrices();
    }

    pub fn move_rel(&mut self, x: f32, y: f32, z: f32) {
        self.pos.x += x;
        self.pos.y += y;
        self.pos.z += z;
        self.update_matrices();
    }

    pub fn get_ray(&self, u: usize, v: usize, mut rng: impl Rng) -> Ray {
        let dir_x = -self.max_x
            + 2.0 * self.max_x * ((u as f32 + rng.gen_range(0.0..1.0)) / self.width as f32);
        let dir_y = -self.max_y
            + 2.0 * self.max_y * ((v as f32 + rng.gen_range(0.0..1.0)) / self.height as f32);
        let dir = Vec4::new(dir_x, -dir_y, 1.0, 1.0);
        let dir = self.rotation_matrix * dir;

        let pos = self.orientation_matrix * Vec4::new(0.0, 0.0, 0.0, 1.0);
        Ray::new(pos.into(), dir.into())
    }

    fn update_matrices(&mut self) {
        self.rotation_matrix = Matrix::rot_x(self.x_angle_radians)
            * Matrix::rot_y(self.y_angle_radians)
            * self.base_rotation_matrix;
        self.orientation_matrix =
            self.rotation_matrix * Matrix::translate(&self.pos) * self.base_orientation_matrix;
    }
}
