use crate::scene::color::RGBA;

pub fn simple_map(color: &RGBA) -> RGBA {
    RGBA::new(
        color.r / (1.0 + color.r),
        color.g / (1.0 + color.g),
        color.b / (1.0 + color.b),
        color.a / (1.0 + color.a)
    )
}

