use crate::linalg::V4;
use crate::ray::Ray;
use crate::shape::BaseShape;

pub struct Plane ();

impl BaseShape for Plane {
    fn intersect(&self, r: &Ray) -> Vec<f32> {
        if r.direction.y().abs() < 0.0001 {
            vec![]
        } else {
            vec![ -r.origin.y() / r.direction.y() ]
        }
    }

    fn normal_at(&self, _: V4) -> V4 {
        V4::new_vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::linalg::V4;

    #[test]
    fn normal() {
        let p = Plane();

        assert_eq!(p.normal_at(V4::new_point(0.0, 0.0, 0.0)), V4::new_vector(0.0, 1.0, 0.0));
        assert_eq!(p.normal_at(V4::new_point(10.0, 0.0, -50.0)), V4::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_parallel() {
        let mut ray = Ray {
            origin: V4::new_point(0.0, 10.0, 0.0),
            direction: V4::new_vector(0.0, 0.0, 1.0)
        };

        assert!(Plane().intersect(&ray).is_empty());

        ray.origin = V4::new_point(0.0, 0.0, 0.0);
        assert!(Plane().intersect(&ray).is_empty());
    }

    #[test]
    fn intersect_above() {
        let ray = Ray {
            origin: V4::new_point(0.0, 1.0, 0.0),
            direction: V4::new_point(0.0, -1.0, 0.0)
        };

        assert_eq!(Plane().intersect(&ray), [ 1.0 ]);
    }

    #[test]
    fn intersect_below() {
        let ray = Ray {
            origin: V4::new_point(0.0, -1.0, 0.0),
            direction: V4::new_point(0.0, 1.0, 0.0)
        };

        assert_eq!(Plane().intersect(&ray), [ 1.0 ]);
    }
}