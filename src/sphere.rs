use crate::lighting::Material;
use crate::linalg;
use crate::linalg::{M4,V4};
use crate::ray::Ray;
use crate::shape::Shape;

pub struct Sphere {
    material: Material,
    transform: M4
}

impl Sphere {
    pub fn new(m: &Material, trans: &M4) -> Sphere {
        Sphere {
            material: *m,
            transform: trans.invert()
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, r: &Ray) -> Vec<f32> {
        let r = r.apply(&self.transform);

        let s2r = r.origin - V4::make_point(0.0, 0.0, 0.0);

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

    fn material(&self) -> Material {
        self.material
    }

    fn normal_at(&self, p: V4) -> V4 {
        let p = linalg::mvmul(&self.transform, &p);
        let t = self.transform.transpose();
        let n = p - V4::make_point(0.0, 0.0, 0.0);
        let n = linalg::mvmul(&t, &n);

        V4::make_vector(n.x(), n.y(), n.z()).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::transform::Transform;
    use float_cmp::*;

    const DEFAULT_MAT: Material = Material {
        color: Color { r: 1.0, g: 0.2, b: 1.0 },
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0
    };

    fn default_sphere() -> Sphere {
        Sphere {
            material: DEFAULT_MAT,
            transform: M4::identity()
        }
    }
    
    #[test]
    fn sphere_intersect() {
        let z = V4::make_vector(0.0, 0.0, 1.0);
        let r = Ray {
            origin: V4::make_point(0.0, 0.0, -5.0), direction: z
        };
        let s = default_sphere();

        assert_eq!(s.intersect(&r), [ 4.0, 6.0 ]);

        let r = Ray {
            origin: V4::make_point(0.0, 1.0, -5.0), direction: z
        };

        assert_eq!(s.intersect(&r), [ 5.0, 5.0 ]);
    }

    #[test]
    fn sphere_normal() {
        let t = Transform::new().translate(0.0, 1.0, 0.0);
        let s = Sphere::new(&DEFAULT_MAT, &t.matrix);

        let p = V4::make_point(0.0, 1.70711, -0.70711);
        let n = V4::make_vector(0.0, 0.70711, -0.70711);

        assert!(approx_eq!(V4, s.normal_at(p), n, epsilon = 0.0001));
    }
}
