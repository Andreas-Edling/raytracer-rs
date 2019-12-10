
#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}
impl Ray {
    pub const fn new(pos: Vec3, dir: Vec3) -> Self {
        Ray {
            pos,
            dir,
        }
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn normalized(&self) -> Vec3 {
        let len = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        Vec3::new(self.x / len, self.y / len, self.z / len)
    }
}

impl From<Vec4> for Vec3 {
    fn from(v: Vec4) -> Self {
        Vec3::new(v.x, v.y, v.z)
    }
}

#[rustfmt::skip] impl std::ops::Add<&Vec3> for &Vec3 { type Output = Vec3; fn add(self, other: &Vec3) -> Vec3 { Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z) }}
#[rustfmt::skip] impl std::ops::Add<&Vec3> for  Vec3 { type Output = Vec3; fn add(self, other: &Vec3) -> Vec3 { Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z) }}
#[rustfmt::skip] impl std::ops::Add< Vec3> for &Vec3 { type Output = Vec3; fn add(self, other: Vec3) -> Vec3 { Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z) }}
#[rustfmt::skip] impl std::ops::Add< Vec3> for  Vec3 { type Output = Vec3; fn add(self, other: Vec3) -> Vec3 { Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z) }}

#[rustfmt::skip] impl std::ops::Sub<&Vec3> for &Vec3 { type Output = Vec3; fn sub(self, other: &Vec3) -> Vec3 { Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z) }}
#[rustfmt::skip] impl std::ops::Sub<&Vec3> for  Vec3 { type Output = Vec3; fn sub(self, other: &Vec3) -> Vec3 { Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z) }}
#[rustfmt::skip] impl std::ops::Sub< Vec3> for &Vec3 { type Output = Vec3; fn sub(self, other: Vec3) -> Vec3 { Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z) }}
#[rustfmt::skip] impl std::ops::Sub< Vec3> for  Vec3 { type Output = Vec3; fn sub(self, other: Vec3) -> Vec3 { Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z) }}

#[rustfmt::skip] impl std::ops::Mul<f32> for &Vec3 { type Output = Vec3; fn mul(self, other: f32) -> Vec3 { Vec3::new(self.x * other, self.y * other, self.z * other) }}
#[rustfmt::skip] impl std::ops::Mul<f32> for  Vec3 { type Output = Vec3; fn mul(self, other: f32) -> Vec3 { Vec3::new(self.x * other, self.y * other, self.z * other) }}
#[rustfmt::skip] impl std::ops::Mul< Vec3> for f32 { type Output = Vec3; fn mul(self, other: Vec3) -> Vec3 { Vec3::new(self * other.x, self * other.y, self * other.z) }}
#[rustfmt::skip] impl std::ops::Mul<&Vec3> for f32 { type Output = Vec3; fn mul(self, other: &Vec3) -> Vec3 { Vec3::new(self * other.x, self * other.y, self * other.z) }}

#[rustfmt::skip] impl std::ops::Neg for Vec3 { type Output = Vec3; fn neg(self) -> Self::Output { Vec3::new(-self.x, -self.y, -self.z) }}



#[derive(PartialEq, Debug, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn from_vec3(v3: &Vec3) -> Self {
        Vec4 {
            x: v3.x,
            y: v3.y,
            z: v3.z,
            w: 1.0,
        }
    }
}

pub fn dot(v0: &Vec3, v1: &Vec3) -> f32 {
    v0.x * v1.x + v0.y * v1.y + v0.z * v1.z
}

#[rustfmt::skip]
pub fn cross(v0: &Vec3, v1: &Vec3) -> Vec3 {
    Vec3::new( 
         v0.y*v1.z - v0.z*v1.y, 
         v0.z*v1.x - v0.x*v1.z, 
         v0.x*v1.y - v0.y*v1.x, 
    )
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Matrix {
    e: [f32; 16],
}
impl Matrix {
    pub fn new(elems: &[f32;16]) -> Self {
        Matrix { e: *elems }
    }

    pub fn from_slice(elems: &[f32]) -> Option<Self> {
        if elems.len()<16 {
            return None;
        }

        let mut array = [0.0; 16];
        let elems = &elems[..16];
        array.copy_from_slice(elems); 
        Some(Matrix{ e: array })
    }

    pub fn ident() -> Self {
        let mut m = Matrix { e: [0.0; 16] };
        m.e[0] = 1.0;
        m.e[5] = 1.0;
        m.e[10] = 1.0;
        m.e[15] = 1.0;
        m
    }

    pub fn rot_x(radians: f32) -> Self {
        let mut m = Matrix::ident();
        m.e[5] = radians.cos();
        m.e[6] = -radians.sin();
        m.e[9] = radians.sin();
        m.e[10] = radians.cos();
        m
    }
    pub fn rot_y(radians: f32) -> Self {
        let mut m = Matrix::ident();
        m.e[0] = radians.cos();
        m.e[2] = radians.sin();
        m.e[8] = -radians.sin();
        m.e[10] = radians.cos();
        m
    }

    pub fn translate(v: &Vec3) -> Self {
        let mut m = Matrix::ident();
        m.e[12] = v.x;
        m.e[13] = v.y;
        m.e[14] = v.z;
        m
    }

    pub fn transpose(&self) -> Self {
        let mut m = *self;
        m.e[1] = self.e[4];
        m.e[2] = self.e[8];
        m.e[3] = self.e[12];

        m.e[4] = self.e[1];
        m.e[6] = self.e[9];
        m.e[7] = self.e[13];

        m.e[8] = self.e[2];
        m.e[9] = self.e[6];
        m.e[11] = self.e[14];

        m.e[12] = self.e[3];
        m.e[13] = self.e[7];
        m[14] = self.e[11];
        m
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl std::fmt::Display for Matrix {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {} {} {}\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}", 
            self.e[0],self.e[1],self.e[2],self.e[3], 
            self.e[4],self.e[5],self.e[6],self.e[7], 
            self.e[8],self.e[9],self.e[10],self.e[11], 
            self.e[12],self.e[13],self.e[14],self.e[15], 
        )
    }
}
impl std::ops::Mul<&Vec4> for &Matrix {
    type Output = Vec4;

    fn mul(self, v: &Vec4) -> Self::Output {
        let x = v.x * self.e[0] + v.y * self.e[4] + v.z * self.e[8] + v.w * self.e[12];
        let y = v.x * self.e[1] + v.y * self.e[5] + v.z * self.e[9] + v.w * self.e[13];
        let z = v.x * self.e[2] + v.y * self.e[6] + v.z * self.e[10] + v.w * self.e[14];
        let w = v.x * self.e[3] + v.y * self.e[7] + v.z * self.e[11] + v.w * self.e[15];
        Vec4::new(x, y, z, w)
    }
}
impl std::ops::Mul<Vec4> for Matrix {
    type Output = Vec4;

    fn mul(self, v: Vec4) -> Self::Output {
        &self * &v
    }
}
impl std::ops::Mul<Vec4> for &Matrix {
    type Output = Vec4;

    fn mul(self, v: Vec4) -> Self::Output {
        self * &v
    }
}
impl std::ops::Mul<&Vec4> for Matrix {
    type Output = Vec4;

    fn mul(self, v: &Vec4) -> Self::Output {
        &self * v
    }
}

impl std::ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let a = self.e[0] * rhs.e[0]
            + self.e[1] * rhs.e[4]
            + self.e[2] * rhs.e[8]
            + self.e[3] * rhs.e[12];
        let b = self.e[0] * rhs.e[1]
            + self.e[1] * rhs.e[5]
            + self.e[2] * rhs.e[9]
            + self.e[3] * rhs.e[13];
        let c = self.e[0] * rhs.e[2]
            + self.e[1] * rhs.e[6]
            + self.e[2] * rhs.e[10]
            + self.e[3] * rhs.e[14];
        let d = self.e[0] * rhs.e[3]
            + self.e[1] * rhs.e[7]
            + self.e[2] * rhs.e[11]
            + self.e[3] * rhs.e[15];

        let e = self.e[4] * rhs.e[0]
            + self.e[5] * rhs.e[4]
            + self.e[6] * rhs.e[8]
            + self.e[7] * rhs.e[12];
        let f = self.e[4] * rhs.e[1]
            + self.e[5] * rhs.e[5]
            + self.e[6] * rhs.e[9]
            + self.e[7] * rhs.e[13];
        let g = self.e[4] * rhs.e[2]
            + self.e[5] * rhs.e[6]
            + self.e[6] * rhs.e[10]
            + self.e[7] * rhs.e[14];
        let h = self.e[4] * rhs.e[3]
            + self.e[5] * rhs.e[7]
            + self.e[6] * rhs.e[11]
            + self.e[7] * rhs.e[15];

        let i = self.e[8] * rhs.e[0]
            + self.e[9] * rhs.e[4]
            + self.e[10] * rhs.e[8]
            + self.e[11] * rhs.e[12];
        let j = self.e[8] * rhs.e[1]
            + self.e[9] * rhs.e[5]
            + self.e[10] * rhs.e[9]
            + self.e[11] * rhs.e[13];
        let k = self.e[8] * rhs.e[2]
            + self.e[9] * rhs.e[6]
            + self.e[10] * rhs.e[10]
            + self.e[11] * rhs.e[14];
        let l = self.e[8] * rhs.e[3]
            + self.e[9] * rhs.e[7]
            + self.e[10] * rhs.e[11]
            + self.e[11] * rhs.e[15];

        let m = self.e[12] * rhs.e[0]
            + self.e[13] * rhs.e[4]
            + self.e[14] * rhs.e[8]
            + self.e[15] * rhs.e[12];
        let n = self.e[12] * rhs.e[1]
            + self.e[13] * rhs.e[5]
            + self.e[14] * rhs.e[9]
            + self.e[15] * rhs.e[13];
        let o = self.e[12] * rhs.e[2]
            + self.e[13] * rhs.e[6]
            + self.e[14] * rhs.e[10]
            + self.e[15] * rhs.e[14];
        let p = self.e[12] * rhs.e[3]
            + self.e[13] * rhs.e[7]
            + self.e[14] * rhs.e[11]
            + self.e[15] * rhs.e[15];
        Matrix {
            e: [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p],
        }
    }
}

impl std::ops::Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self * &rhs
    }
}

impl std::ops::Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        &self * &rhs
    }
}

mod tests {
    #[test]
    fn test_mul_identities() {
        use super::Matrix;
        let m0 = Matrix::ident();
        let m1 = Matrix::ident();
        let m2 = m0 * m1;
        assert_eq!(m2, Matrix::ident());
    }

    #[test]
    fn test_mul_vec_mat() {
        use super::{Matrix, Vec4};
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let m = Matrix::ident();
        let res = m * v;
        assert_eq!(res, Vec4::new(1.0, 2.0, 3.0, 4.0));
    }
}
