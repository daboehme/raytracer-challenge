use crate::linalg::V4;
use crate::ray::Ray;
use crate::shape::BaseShape;

pub struct Cylinder {
    min: f32,
    max: f32,
    is_closed: bool
}

impl Cylinder {
    pub fn new() -> Cylinder {
        Cylinder { min: std::f32::MIN, max: std::f32::MAX, is_closed: false }
    }

    pub fn new_closed(min: f32, max: f32) -> Cylinder {
        Cylinder { min: min, max: max, is_closed: true }
    }

    pub fn new_truncated(min: f32, max: f32) -> Cylinder {
        Cylinder { min: min, max: max, is_closed: false }
    }
}

fn check_cap(y: f32, ray: &Ray) -> Option<f32> {
    if ray.direction.y().abs() < 0.0001 {
        return None
    }

    let t = (y - ray.origin.y()) / ray.direction.y();

    let x = ray.origin.x() + t * ray.direction.x();
    let z = ray.origin.z() + t * ray.direction.z();

    if x*x + z*z <= 1.0 {
        Some(t)
    } else {
        None
    }
}

impl BaseShape for Cylinder {
    fn intersect(&self, ray: &Ray) -> Vec<f32> {
        let d_x = ray.direction.x();
        let d_z = ray.direction.z();

        let a = d_x*d_x + d_z*d_z;

        let mut ret = vec![];

        if a > 0.0 {
            let o_x = ray.origin.x();
            let o_z = ray.origin.z();

            let b = 2.0*o_x*d_x + 2.0*o_z*d_z;
            let c = o_x*o_x + o_z*o_z - 1.0;
            let d = b*b - 4.0*a*c;

            if d >= 0.0 {
                let t0 = (-b - d.sqrt()) / (2.0*a);
                let t1 = (-b + d.sqrt()) / (2.0*a);

                let tmin = t0.min(t1);
                let tmax = t0.max(t1);

                let y0 = ray.origin.y() + tmin * ray.direction.y();
                let y1 = ray.origin.y() + tmax * ray.direction.y();

                if self.min < y0 && y0 < self.max {
                    ret.push(tmin);
                }
                if self.min < y1 && y1 < self.max {
                    ret.push(tmax)
                }
            }
        }

        if self.is_closed {
            if let Some(t) = check_cap(self.min, ray) {
                ret.push(t)
            }
            if let Some(t) = check_cap(self.max, ray) {
                ret.push(t)
            }
        }

        ret
    }

    fn normal_at(&self, p: V4) -> V4 {
        let d = p.x()*p.x() + p.z()*p.z();

        if d < 1.0 {
            if p.y() >= (self.max - 0.0001) {
                return V4::new_vector(0.0,  1.0, 0.0)
            }
            if p.y() <= (self.min + 0.0001) {
                return V4::new_vector(0.0, -1.0, 0.0)
            }
        }

        V4::new_vector(p.x(), 0.0, p.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::linalg::V4;
    use float_cmp::*;

    #[test]
    fn cyl_intersect_miss() {
        let tests = [
            Ray::new(V4::new_point(1.0, 0.0,  0.0), V4::new_vector(0.0, 1.0, 0.0)),
            Ray::new(V4::new_point(0.0, 0.0,  0.0), V4::new_vector(0.0, 1.0, 0.0)),
            Ray::new(V4::new_point(1.0, 0.0, -5.0), V4::new_vector(1.0, 1.0, 1.0))
        ];

        let c = Cylinder::new();

        for t in &tests {
            assert!(c.intersect(t).is_empty())
        }
    }

    #[test]
    fn cyl_intersect_hit() {
        let tests = [
            (V4::new_point(1.0, 0.0, -5.0), V4::new_vector(0.0, 0.0, 1.0), 5.0, 5.0),
            (V4::new_point(0.0, 0.0, -5.0), V4::new_vector(0.0, 0.0, 1.0), 4.0, 6.0),
            (V4::new_point(0.5, 0.0, -5.0), V4::new_vector(0.1, 1.0, 1.0), 6.80798, 7.08872)
        ];

        let c = Cylinder::new();

        for t in &tests {
            let xs = c.intersect(&Ray::new(t.0, t.1.normalize()));

            assert_eq!(xs.len(), 2);

            assert!(approx_eq!(f32, xs[0], t.2, epsilon = 0.0001));
            assert!(approx_eq!(f32, xs[1], t.3, epsilon = 0.0001));
        }
    }

    #[test]
    fn cyl_truncated() {
        let tests = [
            (V4::new_point(0.0,  1.5,  0.0), V4::new_vector(0.1,  1.0, 0.0), 0),
            (V4::new_point(0.0,  3.0, -5.0), V4::new_vector(0.0,  0.0, 1.0), 0),
            (V4::new_point(0.0,  2.0, -5.0), V4::new_vector(0.0,  0.0, 1.0), 0),
            (V4::new_point(0.0,  1.0, -5.0), V4::new_vector(0.0,  0.0, 1.0), 0),
            (V4::new_point(0.0,  1.5, -2.0), V4::new_vector(0.0,  0.0, 1.0), 2)
        ];

        let c = Cylinder::new_truncated(1.0, 2.0);

        for t in &tests {
            assert_eq!(c.intersect(&Ray::new(t.0, t.1.normalize())).len(), t.2)
        }
    }

    #[test]
    fn cyl_closed() {
        let tests = [
            (V4::new_point(0.0,  3.0,  0.0), V4::new_vector(0.0, -1.0, 0.0), 2),
            (V4::new_point(0.0,  3.0, -2.0), V4::new_vector(0.0, -1.0, 2.0), 2),
            // (V4::new_point(0.0,  4.0, -2.0), V4::new_vector(0.0, -1.0, 1.0), 2),
            (V4::new_point(0.0,  0.0, -2.0), V4::new_vector(0.0,  1.0, 2.0), 2),
            // (V4::new_point(0.0, -1.0, -2.0), V4::new_vector(0.0,  1.0, 1.0), 2)
        ];

        let c = Cylinder::new_closed(1.0, 2.0);

        for t in &tests {
            assert_eq!(c.intersect(&Ray::new(t.0, t.1.normalize())).len(), t.2)
        }
    }

    #[test]
    fn cyl_normal() {
        let c = Cylinder::new();

        assert_eq!(c.normal_at(V4::new_point(1.0, 0.0,  0.0)), V4::new_vector(1.0, 0.0,  0.0));
        assert_eq!(c.normal_at(V4::new_point(0.0, 5.0, -1.0)), V4::new_vector(0.0, 0.0, -1.0));
        assert_eq!(c.normal_at(V4::new_point(1.0, 1.0,  0.0)), V4::new_vector(1.0, 0.0,  0.0));
    }

    #[test]
    fn cyl_closed_normal() {
        let tests = [
            (V4::new_point(0.0, 1.0, 0.0), V4::new_vector(0.0, -1.0, 0.0)),
            (V4::new_point(0.5, 1.0, 0.0), V4::new_vector(0.0, -1.0, 0.0)),
            (V4::new_point(0.0, 1.0, 0.5), V4::new_vector(0.0, -1.0, 0.0)),
            (V4::new_point(0.0, 2.0, 0.0), V4::new_vector(0.0,  1.0, 0.0)),
            (V4::new_point(0.5, 2.0, 0.0), V4::new_vector(0.0,  1.0, 0.0)),
            (V4::new_point(0.0, 2.0, 0.5), V4::new_vector(0.0,  1.0, 0.0))
        ];

        let c = Cylinder::new_closed(1.0, 2.0);

        for t in &tests {
            assert_eq!(c.normal_at(t.0), t.1)
        }
    }
}