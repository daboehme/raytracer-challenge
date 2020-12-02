#[derive(Clone,Copy,Debug,PartialEq)]
pub struct V4 (f32, f32, f32, f32);

impl V4 {
    pub fn make_point(x: f32, y: f32, z: f32) -> V4 {
        V4(x, y, z, 1.0)
    }

    pub fn make_vector(x: f32, y: f32, z: f32) -> V4 {
        V4(x, y, z, 0.0)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn w(&self) -> f32 {
        self.3
    }

    pub fn neg(&self) -> V4 {
        V4(-self.0, -self.1, -self.2, -self.3)
    }

    pub fn magnitude(&self) -> f32 {
        (self.0*self.0 + self.1*self.1 + self.2*self.2 + self.3*self.3).sqrt()
    }

    pub fn normalize(&self) -> V4 {
        let m = self.magnitude();
        V4(self.0/m, self.1/m, self.2/m, self.3/m)
    }

    pub fn mult(&self, f: f32) -> V4 {
        V4(f * self.0, f * self.1, f * self.2, f * self.3)
    }

    pub fn add(a: &V4, b: &V4) -> V4 {
        V4(a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
    }

    pub fn sub(a: &V4, b: &V4) -> V4 {
        V4(a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
    }

    pub fn dot(a: &V4, b: &V4) -> f32 {
        a.0*b.0 + a.1*b.1 + a.2*b.2 + a.3*b.3
    }

    pub fn cross(a: &V4, b: &V4) -> V4 {
        V4::make_vector(a.1*b.2-a.2*b.1, a.2*b.0-a.0*b.2, a.0*b.1-a.1*b.0)
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct M4x4([f32; 16]);

impl M4x4 {
    pub fn identity() -> M4x4 {
        M4x4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    pub fn from_array(a: &[f32; 16]) -> M4x4 {
        M4x4(*a)
    }

    pub fn set(&mut self, row: usize, col: usize, val: f32) {
        self.0[4*row+col] = val
    }

    pub fn at(&self, row: usize, col: usize) -> f32 {
        self.0[4*row+col]
    }

    pub fn transpose(&self) -> M4x4 {
        M4x4([
            self.0[0*4+0], self.0[1*4+0], self.0[2*4+0], self.0[3*4+0],
            self.0[0*4+1], self.0[1*4+1], self.0[2*4+1], self.0[3*4+1],
            self.0[0*4+2], self.0[1*4+2], self.0[2*4+2], self.0[3*4+2],
            self.0[0*4+3], self.0[1*4+3], self.0[2*4+3], self.0[3*4+3]
        ])
    }
}

pub fn mmul(a: &M4x4, b: &M4x4) -> M4x4 {
    let mut c = M4x4([ 0.0; 16 ]);

    for y in 0..4 {
        for x in 0..4 {
            c.0[4*y+x] = 
                a.0[4*y+0] * b.0[0*4+x] +
                a.0[4*y+1] * b.0[1*4+x] +
                a.0[4*y+2] * b.0[2*4+x] +
                a.0[4*y+3] * b.0[3*4+x]
        }
    }

    c
}

pub fn mvmul(m: &M4x4, v: &V4) -> V4 {
    V4(
        m.0[0*4+0] * v.0 + 
        m.0[0*4+1] * v.1 +
        m.0[0*4+2] * v.2 +
        m.0[0*4+3] * v.3,

        m.0[1*4+0] * v.0 + 
        m.0[1*4+1] * v.1 +
        m.0[1*4+2] * v.2 +
        m.0[1*4+3] * v.3,

        m.0[2*4+0] * v.0 + 
        m.0[2*4+1] * v.1 +
        m.0[2*4+2] * v.2 +
        m.0[2*4+3] * v.3,

        m.0[3*4+0] * v.0 + 
        m.0[3*4+1] * v.1 +
        m.0[3*4+2] * v.2 +
        m.0[3*4+3] * v.3
    )
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn make_point() {
        let p = V4::make_point(1.0, -3.5, 9.0);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), -3.5);
        assert_eq!(p.z(), 9.0);
        assert_eq!(p.w(), 1.0);
    }

    #[test]
    fn add_pv() {
        let p = V4::make_point(4.0, -5.5, 10.0);
        let v = V4::make_vector(1.5, 2.0, 3.5);
        let r = V4::add(&p, &v);
        assert_eq!(r.x(), 5.5);
        assert_eq!(r.y(), -3.5);
        assert_eq!(r.z(), 13.5);
    }

    #[test]
    fn magnitude() {
        assert_eq!(V4::make_vector(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(V4::make_vector(-1.0, -2.0, -3.0).magnitude(), (14.0_f32).sqrt());
    }

    #[test]
    fn normalize() {
        let v1n = V4::make_vector(4.0, 0.0, 0.0).normalize();
        assert_eq!(v1n.x(), 1.0);
        assert_eq!(v1n.y(), 0.0);
        assert_eq!(v1n.z(), 0.0);
    }

    #[test]
    fn dot() {
        let a = V4::make_vector(1.0, 2.0, 3.0);
        let b = V4::make_vector(2.0, 3.0, 4.0);
        assert_eq!(V4::dot(&a, &b), 20.0)
    }

    #[test]
    fn cross() {
        let a = V4::make_vector(1.0, 2.0, 3.0);
        let b = V4::make_vector(2.0, 3.0, 4.0);
        let c = V4::cross(&a, &b);
        assert_eq!(c.x(), -1.0);
        assert_eq!(c.y(), 2.0);
        assert_eq!(c.z(), -1.0);
        let c = V4::cross(&b, &a);
        assert_eq!(c.x(), 1.0);
        assert_eq!(c.y(), -2.0);
        assert_eq!(c.z(), 1.0)
    }

    #[test]
    fn matrix_set_at() {
        let mut a = M4x4::identity();
        assert_eq!(a.at(0, 0), 1.0);
        assert_eq!(a.at(0, 1), 0.0);
        assert_eq!(a.at(1, 1), 1.0);
        assert_eq!(a.at(3, 3), 1.0);
        assert_eq!(a.at(2, 3), 0.0);

        a.set(3, 1, 42.0);
        assert_eq!(a.at(3, 1), 42.0);
    }

    #[test]
    fn mm_mul() {
        let a = M4x4([ 
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]);
        let b = M4x4([
            -2.0, 1.0, 2.0,  3.0,
             3.0, 2.0, 1.0, -1.0,
             4.0, 3.0, 6.0,  5.0,
             1.0, 2.0, 7.0,  8.0
        ]);

        let res = M4x4([
            20.0, 22.0,  50.0,  48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0,  46.0,  42.0
        ]);

        assert_eq!(mmul(&a, &b), res);

        let  i = M4x4::identity();
        assert_eq!(mmul(&a, &i), a);
    }

    #[test]
    fn mv_mul() {
        let a = M4x4([
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0
        ]);
        
        let v   = V4(1.0,   2.0,  3.0, 1.0);
        let res = V4(18.0, 24.0, 33.0, 1.0);

        assert_eq!(mvmul(&a, &v), res);
    }

    #[test]
    fn transpose() {
        let m = M4x4([
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0
        ]);
        let t = M4x4([
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0
        ]);

        assert_eq!(m.transpose(), t);
        assert_eq!(M4x4::identity().transpose(), M4x4::identity());
    }
}