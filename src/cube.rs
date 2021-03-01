use crate::linalg::V4;
use crate::ray::Ray;
use crate::shape::BaseShape;

pub struct Cube ();

fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    let tmin_num = -1.0 - origin;
    let tmax_num =  1.0 - origin;

    let (tmin, tmax) = if direction.abs() > 0.0001 {
        (tmin_num / direction, tmax_num / direction)
    } else {
        (tmin_num * f32::INFINITY, tmax_num * f32::INFINITY)
    };

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl BaseShape for Cube {
    fn intersect(&self, r: &Ray) -> Vec<f32> {
        let (xtmin, xtmax) = check_axis(r.origin.x(), r.direction.x());
        let (ytmin, ytmax) = check_axis(r.origin.y(), r.direction.y());
        let (ztmin, ztmax) = check_axis(r.origin.z(), r.direction.z());

        let tmin = [ xtmin, ytmin, ztmin ].iter().fold(f32::MIN, |a, &b| a.max(b));
        let tmax = [ xtmax, ytmax, ztmax ].iter().fold(f32::MAX, |a, &b| a.min(b));

        if tmin > tmax {
            vec![]
        } else {
            vec![ tmin, tmax ]
        }
    }

    fn normal_at(&self, p: V4) -> V4 {
        if p.x().abs() > p.y().abs() {
            if p.x().abs() > p.z().abs() {
                V4::new_vector(p.x(), 0.0, 0.0)
            } else {
                V4::new_vector(0.0, 0.0, p.z())
            }
        } else {
            if p.y().abs() > p.z().abs() {
                V4::new_vector(0.0, p.y(), 0.0)
            } else {
                V4::new_vector(0.0, 0.0, p.z())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::linalg::V4;
    use float_cmp::*;

    #[test]
    fn cube_intersect() {
        let c = Cube();

        let tests = [
            ( V4::new_point( 5.0,  0.5,  0.0), V4::new_vector(-1.0,  0.0,  0.0),  4.0, 6.0 ),
            ( V4::new_point(-5.0,  0.5,  0.0), V4::new_vector( 1.0,  0.0,  0.0),  4.0, 6.0 ),
            ( V4::new_point( 0.5,  5.0,  0.0), V4::new_vector( 0.0, -1.0,  0.0),  4.0, 6.0 ),
            ( V4::new_point( 0.5, -5.0,  0.0), V4::new_vector( 0.0,  1.0,  0.0),  4.0, 6.0 ),
            ( V4::new_point( 0.5,  0.0,  5.0), V4::new_vector( 0.0,  0.0, -1.0),  4.0, 6.0 ),
            ( V4::new_point( 0.5,  0.0, -5.0), V4::new_vector( 0.0,  0.0,  1.0),  4.0, 6.0 ),
            ( V4::new_point( 0.0,  0.5,  0.0), V4::new_vector( 0.0,  0.0,  1.0), -1.0, 1.0 )
        ];

        for x in tests.iter() {
            let xs = c.intersect(&Ray::new(x.0, x.1));

            assert_eq!(xs.len(), 2);

            assert!(approx_eq!(f32, xs[0], x.2, epsilon = 0.0001));
            assert!(approx_eq!(f32, xs[1], x.3, epsilon = 0.0001));
        }
    }

    #[test]
    fn cube_miss() {
        let tests = [
            Ray::new(V4::new_point(-2.0,  0.0,  0.0), V4::new_vector( 0.2673,  0.5345,  0.8018)),
            Ray::new(V4::new_point( 0.0, -2.0,  0.0), V4::new_vector( 0.8018,  0.2673,  0.5345)),
            Ray::new(V4::new_point( 0.0,  0.0, -2.0), V4::new_vector( 0.5345,  0.8018,  0.2673)),
            Ray::new(V4::new_point( 2.0,  0.0,  2.0), V4::new_vector( 0.0,     0.0,    -1.0)),
            Ray::new(V4::new_point( 0.0,  2.0,  2.0), V4::new_vector( 0.0,    -1.0,     0.0)),
            Ray::new(V4::new_point( 2.0,  2.0,  0.0), V4::new_vector(-1.0,     0.0,     0.0)),
        ];

        let c = Cube();

        for x in tests.iter() {
            assert!(c.intersect(x).is_empty());
        }
    }

    #[test]
    fn cube_normal() {
        let c = Cube();

        assert_eq!(c.normal_at(V4::new_point( 1.0,  0.5, -0.8)), V4::new_vector( 1.0,  0.0,  0.0));
        assert_eq!(c.normal_at(V4::new_point(-1.0,  0.2,  0.9)), V4::new_vector(-1.0,  0.0,  0.0));
        assert_eq!(c.normal_at(V4::new_point(-0.4,  1.0, -0.1)), V4::new_vector( 0.0,  1.0,  0.0));
        assert_eq!(c.normal_at(V4::new_point( 0.3, -1.0, -0.8)), V4::new_vector( 0.0, -1.0,  0.0));
        assert_eq!(c.normal_at(V4::new_point( 0.5,  0.5, -1.0)), V4::new_vector( 0.0,  0.0, -1.0));
        assert_eq!(c.normal_at(V4::new_point( 0.5,  0.5,  1.0)), V4::new_vector( 0.0,  0.0,  1.0));
    }
}
