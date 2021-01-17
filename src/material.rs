use crate::color::Color;
use crate::linalg::{M4,V4};
use crate::pattern::{Pattern,TransformedPattern};

use std::rc::Rc;

#[derive(Clone,Debug)]
pub enum Texture {
    Color(Color),
    Pattern(Rc<dyn Pattern>)
}

#[derive(Clone,Debug)]
pub struct Material {
    pub texture: Texture,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32
}

impl Material {
    pub fn new_transformed(from: &Material, transform: &M4) -> Material {
        let mut mat = from.clone();

        if let Texture::Pattern(p) = mat.texture {
            mat.texture = Texture::Pattern(Rc::new(TransformedPattern::new_from_rc(p, transform)))
        }

        mat
    }

    pub fn color_at(&self, point: V4) -> Color {
        match &self.texture {
            Texture::Color(c) => *c,
            Texture::Pattern(p) => p.color_at(point)
        }
    }
}
