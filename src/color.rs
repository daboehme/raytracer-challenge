use crate::linalg::V4;

use image::Rgb;
use std::convert::From;


#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };
    pub const RED:   Color = Color { r: 1.0, g: 0.0, b: 0.0 };
}

impl From<V4> for Color {
    fn from(v: V4) -> Color {
        Color { r: v.x(), g: v.y(), b: v.z() }
    }
}

impl From<Color> for V4 {
    fn from(c: Color) -> V4 {
        V4::new_vector(c.r, c.g, c.b)
    }
}

impl From<Color> for Rgb<u8> {
    fn from(c: Color) -> Rgb<u8> {
        fn to_u8(v: f32) -> u8 {
            unsafe {
                (v.max(0.0).min(1.0) * 255.0).to_int_unchecked::<u8>()
            }
        };

        image::Rgb([to_u8(c.r), to_u8(c.g), to_u8(c.b)])
    }
}
