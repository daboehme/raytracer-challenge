use crate::linalg::V4;
use crate::ray::Ray;
use crate::shape::BaseShape;

pub struct Sphere ();

impl BaseShape for Sphere {
    fn intersect(&self, r: &Ray) -> Vec<f32> {
        let s2r = r.origin - V4::new_point(0.0, 0.0, 0.0);

        let a = V4::dot(&r.direction, &r.direction);
        let b = 2.0 * V4::dot(&r.direction, &s2r);
        let c = V4::dot(&s2r, &s2r) - 1.0;

        let d = b*b - 4.0*a*c;

        let mut v: Vec<f32> = vec![];

        if d < 0.0 {
            return v;
        }

        v.push( (-b - d.sqrt()) / (2.0*a) );
        v.push( (-b + d.sqrt()) / (2.0*a) );

        v
    }

    fn normal_at(&self, p: V4) -> V4 {
        p - V4::new_point(0.0, 0.0, 0.0).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::{Material,Texture};
    use crate::linalg::{M4,V4};
    use crate::transform::Transform;
    use crate::shape::Shape;
    use float_cmp::*;

    const DEFAULT_MAT: Material = Material {
        texture: Texture::Color(Color { r: 1.0, g: 0.2, b: 1.0 }),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.0
    };

    fn default_sphere() -> Shape {
        Shape::new(Box::new(Sphere()), &DEFAULT_MAT, &M4::identity())
    }

    #[test]
    fn sphere_intersect() {
        let z = V4::new_vector(0.0, 0.0, 1.0);
        let r = Ray {
            origin: V4::new_point(0.0, 0.0, -5.0), direction: z
        };
        let s = default_sphere();

        assert_eq!(s.intersect(&r), [ 4.0, 6.0 ]);

        let r = Ray {
            origin: V4::new_point(0.0, 1.0, -5.0), direction: z
        };

        assert_eq!(s.intersect(&r), [ 5.0, 5.0 ]);
    }

    #[test]
    fn sphere_normal() {
        let t = Transform::new().translate(0.0, 1.0, 0.0);
        let s = Shape::new(Box::new(Sphere()), &DEFAULT_MAT, &t.matrix);

        let p = V4::new_point(0.0, 1.70711, -0.70711);
        let n = V4::new_vector(0.0, 0.70711, -0.70711);

        assert!(approx_eq!(V4, s.normal_at(p), n, epsilon = 0.0001));
    }
}
