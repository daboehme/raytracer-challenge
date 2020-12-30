use crate::linalg::V4;
use crate::lighting::Material;
use crate::ray::Ray;

pub trait Shape {
    fn intersect(&self, r: &Ray) -> Vec<f32>;
    fn material(&self) -> Material;
    fn normal_at(&self, p: V4) -> V4;
}
