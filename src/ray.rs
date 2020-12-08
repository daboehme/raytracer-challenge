use crate::linalg;

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Ray {
    pub origin: linalg::V4,
    pub direction: linalg::V4
}

impl Ray {
    pub fn apply(&self, m: &linalg::M4) -> Ray {
        Ray {
            origin: linalg::mvmul(m, &self.origin),
            direction: linalg::mvmul(m, &self.direction)
        }
    }

    pub fn position(&self, t: f32) -> linalg::V4 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use linalg::*;
    use crate::transform::*;
    use float_cmp::*;

    #[test]
    fn transform() {
        let trans = Transform::new().translate(3.0, 4.0, 5.0);

        let ray = Ray {
            origin: V4::make_point(1.0, 2.0, 3.0),
            direction: V4::make_vector(0.0, 1.0, 0.0)
        };

        let rt = ray.apply(&trans.matrix);

        assert!(approx_eq!(V4, rt.origin, V4::make_point(4.0, 6.0, 8.0)));
        assert!(approx_eq!(V4, rt.direction, V4::make_vector(0.0, 1.0, 0.0)));
    }

    #[test]
    fn position() {
        let ray = Ray {
            origin: V4::make_point(2.0, 3.0, 4.0),
            direction: V4::make_vector(1.0, 0.0, 0.0)
        };

        assert_eq!(ray.position(2.5), V4::make_point(4.5, 3.0, 4.0));
    }
}
