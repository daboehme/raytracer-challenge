use crate::linalg;
use crate::ray::Ray;

pub trait SceneObject {
    fn intersect(&self, r: &Ray) -> Vec<f32>;
}

pub struct Sphere {
    origin: linalg::V4
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere { origin: linalg::V4::make_point(0.0, 0.0, 0.0) }
    }
}

impl SceneObject for Sphere {
    fn intersect(&self, r: &Ray) -> Vec<f32> {
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
}
