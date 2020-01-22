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
        0.4124564 * color.r + 0.3575761 * color.g + 0.1804375 * color.b,
        0.2126729 * color.r + 0.7151522 * color.g + 0.0721750 * color.b,
        0.0193339 * color.r + 0.1191920 * color.g + 0.9503041 * color.b,
    )
}

fn to_rgb(color: &XYZ) -> RGB {
    RGB::new(
        3.2404542 * color.r + -1.5371385 * color.g + -0.4985314 * color.b,
        -0.9692660 * color.r + 1.8760108 * color.g + 0.0415560 * color.b,
        0.0556434 * color.r + -0.2040259 * color.g + 1.0572252 * color.b,
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
