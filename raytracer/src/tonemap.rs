use crate::scene::color::RGB;

#[allow(dead_code)]
pub fn simple_map(color: &RGB) -> RGB {
    RGB::new(
        color.r / (1.0 + color.r),
        color.g / (1.0 + color.g),
        color.b / (1.0 + color.b),
    )
}

#[allow(dead_code)]
pub fn luminance_simple_map(color: &RGB) -> RGB {
    let mut xyz = to_xyz(color);

    // tonemap luminance channel
    xyz.g = xyz.g / (1.0 + xyz.g);

    to_rgb(&xyz)
}

#[allow(dead_code)]
pub fn gamma_map(color: &RGB) -> RGB {
    const A: f32 = 0.5; // [0..inf)
    const GAMMA: f32 = 0.5; // [0..1]

    let mut xyz = to_xyz(color);

    // gamma compress luminance channel
    xyz.g = A * xyz.g.powf(GAMMA);

    to_rgb(&xyz)
}

type XYZ = RGB;

fn to_xyz(color: &RGB) -> XYZ {
    XYZ::new(
        0.412_456_4 * color.r + 0.357_576_1 * color.g + 0.180_437_5 * color.b,
        0.212_672_9 * color.r + 0.715_152_2 * color.g + 0.072_175_0 * color.b,
        0.019_333_9 * color.r + 0.119_192 * color.g + 0.950_304_1 * color.b,
    )
}

fn to_rgb(color: &XYZ) -> RGB {
    RGB::new(
        3.240_454_2 * color.r + -1.537_138_5 * color.g + -0.498_531_4 * color.b,
        -0.969_266 * color.r + 1.876_010_8 * color.g + 0.041_556_0 * color.b,
        0.055_643_4 * color.r + -0.204_025_9 * color.g + 1.057_225_2 * color.b,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;

    #[test]
    fn test_color_spaces() {
        let rgb = RGB::new(1.0, 1.0, 1.0);
        let transformed = to_rgb(&to_xyz(&rgb));

        let diff_r = (rgb.r.abs() - transformed.r).abs();
        let diff_g = (rgb.g.abs() - transformed.g).abs();
        let diff_b = (rgb.b.abs() - transformed.b).abs();
        assert!(diff_r <= f32::EPSILON);
        assert!(diff_g <= f32::EPSILON);
        assert!(diff_b <= f32::EPSILON);
    }
}
