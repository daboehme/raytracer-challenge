use crate::linalg;
use crate::linalg::{M4,V4};

#[derive(Clone,Copy,Debug)]
pub struct Transform {
    pub matrix: linalg::M4
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            matrix: M4::identity()
        }
    }

    pub fn view_transform(from: &V4, to: &V4, up: &V4) -> Transform {
        let forward = (*to - *from).normalize();
        let upn = up.normalize();
        let left = V4::cross(&forward, &upn);
        let upt = V4::cross(&left, &forward);

        let orientation = [
             left.x(),     left.y(),     left.z(),    0.0,
             upt.x(),      upt.y(),      upt.z(),     0.0,
            -forward.x(), -forward.y(), -forward.z(), 0.0,
             0.0,          0.0,          0.0,         1.0
        ].into();
        let translation = [
            1.0, 0.0, 0.0, -from.x(),
            0.0, 1.0, 0.0, -from.y(),
            0.0, 0.0, 1.0, -from.z(),
            0.0, 0.0, 0.0,  1.0
        ].into();

        Transform {
            matrix: M4::mmul(&orientation, &translation)
        }
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Transform {
        let transmatrix = [
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0
        ].into();

        Transform {
            matrix: M4::mmul(&self.matrix, &transmatrix)
        }
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Transform {
        let transmatrix = [
            x,   0.0, 0.0, 0.0,
            0.0, y,   0.0, 0.0,
            0.0, 0.0, z,   0.0,
            0.0, 0.0, 0.0, 1.0
        ].into();

        Transform {
            matrix: M4::mmul(&self.matrix, &transmatrix)
        }
    }

    pub fn rotate_x(&self, rad: f32) -> Transform {
        let transmatrix = [
            1.0, 0.0,        0.0,       0.0,
            0.0, rad.cos(), -rad.sin(), 0.0,
            0.0, rad.sin(),  rad.cos(), 0.0,
            0.0, 0.0,        0.0,       1.0
        ].into();

        Transform {
            matrix: M4::mmul(&self.matrix, &transmatrix)
        }
    }

    pub fn rotate_y(&self, rad: f32) -> Transform {
        let transmatrix = [
             rad.cos(), 0.0, rad.sin(), 0.0,
             0.0,       1.0, 0.0,       0.0,
            -rad.sin(), 0.0, rad.cos(), 0.0,
             0.0,       0.0, 0.0,       1.0
        ].into();

        Transform {
            matrix: M4::mmul(&self.matrix, &transmatrix)
        }
    }

    pub fn rotate_z(&self, rad: f32) -> Transform {
        let transmatrix = [
            rad.cos(), -rad.sin(), 0.0, 0.0,
            rad.sin(),  rad.cos(), 0.0, 0.0,
            0.0,        0.0,       1.0, 0.0,
            0.0,        0.0,       0.0, 1.0
        ].into();

        Transform {
            matrix: M4::mmul(&self.matrix, &transmatrix)
        }
    }

    pub fn invert(&self) -> Transform {
        Transform {
            matrix: self.matrix.invert()
        }
    }

    pub fn apply(&self, v: V4) -> V4 {
        &self.matrix * v
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use linalg::*;
    use float_cmp::*;

    #[test]
    fn translate() {
        let trans = Transform::new().translate(5.0, -3.0, 2.0);

        let p = V4::new_point(-3.0, 4.0, 5.0);
        let v = V4::new_vector(-3.0, 4.0, 5.0);

        assert!(approx_eq!(V4, trans.apply(p), V4::new_point(2.0, 1.0, 7.0), epsilon = 0.0001));
        assert_eq!(trans.apply(v), v) // translate shouldn't affect vectors
    }

    #[test]
    fn scale() {
        let trans = Transform::new().scale(2.0, 3.0, 4.0);

        let p = V4::new_point(-4.0, 6.0, 8.0);
        let v = V4::new_vector(-4.0, 6.0, 8.0);

        assert!(approx_eq!(V4, trans.apply(p), V4::new_point(-8.0, 18.0, 32.0), epsilon = 0.0001));
        assert!(approx_eq!(V4, trans.apply(v), V4::new_vector(-8.0, 18.0, 32.0), epsilon = 0.0001));

        let trans = trans.invert();
        assert!(approx_eq!(V4, trans.apply(v), V4::new_vector(-2.0, 2.0, 2.0), epsilon = 0.0001));
    }

    #[test]
    fn reflect() {
        let trans = Transform::new().scale(-1.0, 1.0, 1.0);
        let p = V4::new_point(2.0, 3.0, 4.0);

        assert!(approx_eq!(V4, trans.apply(p), V4::new_point(-2.0, 3.0, 4.0), epsilon = 0.0001));
    }

    #[test]
    fn rotate_x() {
        let half_q = Transform::new().rotate_x(std::f32::consts::FRAC_PI_4);
        let full_q = Transform::new().rotate_x(std::f32::consts::FRAC_PI_2);

        let sq2half = std::f32::consts::SQRT_2 / 2.0;

        let p = V4::new_point(0.0, 1.0, 0.0);

        let res_h = V4::new_point(0.0, sq2half, sq2half);
        let res_f = V4::new_point(0.0, 0.0, 1.0);

        assert!(approx_eq!(V4, half_q.apply(p), res_h, epsilon = 0.0001));
        assert!(approx_eq!(V4, full_q.apply(p), res_f, epsilon = 0.0001));
    }

    #[test]
    fn rotate_y() {
        let half_q = Transform::new().rotate_y(std::f32::consts::FRAC_PI_4);
        let full_q = Transform::new().rotate_y(std::f32::consts::FRAC_PI_2);

        let sq2half = std::f32::consts::SQRT_2 / 2.0;

        let p = V4::new_point(0.0, 0.0, 1.0);

        let res_h = V4::new_point(sq2half, 0.0, sq2half);
        let res_f = V4::new_point(1.0, 0.0, 0.0);

        assert!(approx_eq!(V4, half_q.apply(p), res_h));
        assert!(approx_eq!(V4, full_q.apply(p), res_f));
    }

    #[test]
    fn rotate_z() {
        let half_q = Transform::new().rotate_z(std::f32::consts::FRAC_PI_4);
        let full_q = Transform::new().rotate_z(std::f32::consts::FRAC_PI_2);

        let sq2half = std::f32::consts::SQRT_2 / 2.0;

        let p = V4::new_point(0.0, 1.0, 0.0);

        let res_h = V4::new_point(-sq2half, sq2half, 0.0);
        let res_f = V4::new_point(-1.0, 0.0, 0.0);

        assert!(approx_eq!(V4, half_q.apply(p), res_h, epsilon = 0.0001));
        assert!(approx_eq!(V4, full_q.apply(p), res_f, epsilon = 0.0001));
    }

    #[test]
    fn chain() {
        let p = V4::new_point(1.0, 0.0, 1.0);

        let a = Transform::new().rotate_x(std::f32::consts::FRAC_PI_2);
        let b = Transform::new().scale(5.0, 5.0, 5.0);
        let c = Transform::new().translate(10.0, 5.0, 7.0);

        let q = a.apply(p);
        let q = b.apply(q);
        let q = c.apply(q);

        let r = V4::new_point(15.0, 0.0, 7.0);

        assert!(approx_eq!(V4, q, r));

        let t = Transform::new()
            .translate(10.0, 5.0, 7.0)
            .scale(5.0, 5.0, 5.0)
            .rotate_x(std::f32::consts::FRAC_PI_2);

        let q = t.apply(p);

        assert!(approx_eq!(V4, q, r));
    }

    #[test]
    fn view_transform_default() {
        let from = V4::new_point(0.0, 0.0, 0.0);
        let to = V4::new_point(0.0, 0.0, -1.0);
        let up = V4::new_vector(0.0, 1.0, 0.0);

        let t = Transform::view_transform(&from, &to, &up);
        let i = M4::identity();

        assert!(approx_eq!(&M4, &t.matrix, &i));
    }

    #[test]
    fn view_transform_scale() {
        let from = V4::new_point(0.0, 0.0, 0.0);
        let to = V4::new_point(0.0, 0.0, 1.0);
        let up = V4::new_vector(0.0, 1.0, 0.0);

        let tv = Transform::view_transform(&from, &to, &up);
        let ts = Transform::new().scale(-1.0, 1.0, -1.0);

        assert!(approx_eq!(&M4, &tv.matrix, &ts.matrix));
    }

    #[test]
    fn view_transform_move() {
        let from = V4::new_point(0.0, 0.0, 8.0);
        let to = V4::new_point(0.0, 0.0, 0.0);
        let up = V4::new_vector(0.0, 1.0, 0.0);

        let tv = Transform::view_transform(&from, &to, &up);
        let tt = Transform::new().translate(0.0, 0.0, -8.0);

        assert!(approx_eq!(&M4, &tv.matrix, &tt.matrix));
    }

    #[test]
    fn view_transform_misc() {
        let from = V4::new_point(1.0, 3.0, 2.0);
        let to = V4::new_point(4.0, -2.0, 8.0);
        let up = V4::new_vector(1.0, 1.0, 0.0);

        let tv = Transform::view_transform(&from, &to, &up);
        let m = [
            -0.50709, 0.50709,  0.67612, -2.36653,
             0.76772, 0.60609,  0.12122, -2.82843,
            -0.35857, 0.59761, -0.71714,  0.00000,
             0.00000, 0.00000,  0.00000,  1.00000
        ].into();

        assert!(approx_eq!(&M4, &tv.matrix, &m, epsilon=0.0001));
    }
}
