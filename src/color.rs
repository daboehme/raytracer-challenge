use crate::linalg::V4;

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
        V4::make_vector(c.r, c.g, c.b)
    }
}
