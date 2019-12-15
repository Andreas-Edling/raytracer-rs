#[derive(Debug,Copy,Clone)]
pub struct RGB {
    r: f32,
    g: f32,
    b: f32,
}
impl RGB {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
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

pub struct RGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
impl RGBA {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        RGBA{r, g, b, a}
    }

    pub fn from_rgb(rgb: RGB, a: f32) -> Self {
        RGBA::new( rgb.r, rgb.g, rgb.b, a)
    }

    pub fn to_u32(&self) -> u32 {
        let r = (self.r.min(1.0).max(0.0)*255.0) as u8;
        let g = (self.g.min(1.0).max(0.0)*255.0) as u8;
        let b = (self.b.min(1.0).max(0.0)*255.0) as u8;
        let a = (self.a.min(1.0).max(0.0)*255.0) as u8;
        b as u32 | (g as u32) << 8 | (r as u32) << 16 | (a as u32) << 24
        
    }
}

