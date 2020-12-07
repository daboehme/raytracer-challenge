use crate::linalg;
use crate::ray;
use crate::render;

pub trait SceneObject {
    fn intersect(&self, r: &ray::Ray) -> Vec<f32>;
}

pub struct Sphere {
    origin: linalg::V4,
    // material: render::Material,
    transform: linalg::M4
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere { 
            origin: linalg::V4::make_point(0.0, 0.0, 0.0),
            transform: linalg::M4::identity()
        }
    }

    pub fn new_transformed(trans: &linalg::M4) -> Sphere {
        Sphere {
            origin: linalg::V4::make_point(0.0, 0.0, 0.0),
            transform: trans.invert()
        }
    }

    pub fn normal_at(&self, p: linalg::V4) -> linalg::V4 {
        let p = linalg::mvmul(&self.transform, &p);
        let t = self.transform.transpose();
        let n = p - self.origin;
        let n = linalg::mvmul(&t, &n);

        linalg::V4::make_vector(n.x(), n.y(), n.z()).normalize()
    }
}

impl SceneObject for Sphere {
    fn intersect(&self, r: &ray::Ray) -> Vec<f32> {
        let r = r.apply(&self.transform);

        let s2r = r.origin - self.origin;

        let a = linalg::V4::dot(&r.direction, &r.direction);
        let b = 2.0 * linalg::V4::dot(&r.direction, &s2r);
        let c = linalg::V4::dot(&s2r, &s2r) - 1.0;

        let d = b*b - 4.0*a*c;

        let mut v: Vec<f32> = vec![];

        if d < 0.0 {
            return v;
        }

        v.push( (-b - d.sqrt()) / (2.0*a) );
        v.push( (-b + d.sqrt()) / (2.0*a) );

        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use linalg::*;
    use ray::Ray;
    use crate::transform::Transform;
    use float_cmp::*;

    #[test]
    fn sphere_intersect() {
        let z = V4::make_vector(0.0, 0.0, 1.0);
        let r = Ray {
            origin: V4::make_point(0.0, 0.0, -5.0), direction: z
        };
        let s = Sphere::new();

        assert_eq!(s.intersect(&r), [ 4.0, 6.0 ]);

        let r = Ray {
            origin: V4::make_point(0.0, 1.0, -5.0), direction: z
        };

        assert_eq!(s.intersect(&r), [ 5.0, 5.0 ]);
    }

    #[test]
    fn sphere_normal() {
        let t = Transform::new().translate(0.0, 1.0, 0.0);
        let s = Sphere::new_transformed(&t.matrix);

        let p = V4::make_point(0.0, 1.70711, -0.70711);
        let n = V4::make_vector(0.0, 0.70711, -0.70711);

        assert!(approx_eq!(V4, s.normal_at(p), n, epsilon = 0.0001));
    }
}
