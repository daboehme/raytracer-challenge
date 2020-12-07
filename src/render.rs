use crate::linalg;

#[derive(Clone,Copy,Debug)]
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

#[derive(Clone,Copy,Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32
}

#[derive(Clone,Copy,Debug)]
pub struct Light {
    pub intensity: Color,
    pub pos: linalg::V4
}

pub fn lighting
    (
        material: Material, 
        light:    Light,
        point:    linalg::V4,
        eyev:     linalg::V4,
        normalv:  linalg::V4
    ) -> Color 
{
    Color::BLACK
}
