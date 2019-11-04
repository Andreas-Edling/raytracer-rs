

pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub fn dot(v0: &Vec3, v1: &Vec3) -> f32 {
    v0.x*v1.x + v0.y*v1.y + v0.z*v1.z
}

#[rustfmt::skip]
pub fn cross(v0: &Vec3, v1: &Vec3) -> Vec3 {
    Vec3::new( 
         v0.y*v1.z - v0.z*v1.y, 
         v0.z*v1.x - v0.x*v1.z, 
         v0.x*v1.y - v0.y*v1.x, 
    )
}




