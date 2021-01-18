use crate::linalg::{V4,M4};

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Ray {
    pub origin: V4,
    pub direction: V4
}

impl Ray {
    pub fn new(orig: V4, dir: V4) -> Ray {
        Ray {
            origin: orig,
            direction: dir
        }
    }

    pub fn apply(&self, m: &M4) -> Ray {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction
        }
    }

    pub fn position(&self, t: f32) -> V4 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::*;
    use float_cmp::*;

    #[test]
    fn transform() {
        let trans = Transform::new().translate(3.0, 4.0, 5.0);
        let ray = Ray::new(V4::new_point(1.0, 2.0, 3.0), V4::new_vector(0.0, 1.0, 0.0));

        let rt = ray.apply(&trans.matrix);

        assert!(approx_eq!(V4, rt.origin, V4::new_point(4.0, 6.0, 8.0)));
        assert!(approx_eq!(V4, rt.direction, V4::new_vector(0.0, 1.0, 0.0)));
    }

    #[test]
    fn position() {
        let ray = Ray {
            origin: V4::new_point(2.0, 3.0, 4.0),
            direction: V4::new_vector(1.0, 0.0, 0.0)
        };

        assert_eq!(ray.position(2.5), V4::new_point(4.5, 3.0, 4.0));
    }
}
