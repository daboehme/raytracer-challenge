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
pub struct M4([f32; 16]);

impl M4 {
    pub fn identity() -> M4 {
        M4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    pub fn from_array(a: &[f32; 16]) -> M4 {
        M4(*a)
    }

    pub fn set(&mut self, row: usize, col: usize, val: f32) {
        self.0[4*row+col] = val
    }

    pub fn at(&self, row: usize, col: usize) -> f32 {
        self.0[4*row+col]
    }

    pub fn transpose(&self) -> M4 {
        M4([
            self.0[0*4+0], self.0[1*4+0], self.0[2*4+0], self.0[3*4+0],
            self.0[0*4+1], self.0[1*4+1], self.0[2*4+1], self.0[3*4+1],
            self.0[0*4+2], self.0[1*4+2], self.0[2*4+2], self.0[3*4+2],
            self.0[0*4+3], self.0[1*4+3], self.0[2*4+3], self.0[3*4+3]
        ])
    }

    fn minor(&self, row: usize, col: usize) -> f32 {
        fn skip(i: usize) -> [usize;3] {
            match i {
                0  => [1,2,3],
                1  => [0,2,3],
                2  => [0,1,3],
                3  => [0,1,2],
                _  => panic!("M4::minor(): Invalid index")
            }
        }

        let rows = skip(row);
        let cols = skip(col);

        let a = self.0[rows[0]*4+cols[0]];
        let b = self.0[rows[0]*4+cols[1]];
        let c = self.0[rows[0]*4+cols[2]];

        let d = self.0[rows[1]*4+cols[0]];
        let e = self.0[rows[1]*4+cols[1]];
        let f = self.0[rows[1]*4+cols[2]];

        let g = self.0[rows[2]*4+cols[0]];
        let h = self.0[rows[2]*4+cols[1]];
        let i = self.0[rows[2]*4+cols[2]];

        (a*e*i) + (b*f*g) + (c*d*h) - (c*e*g) - (b*d*i) - (a*f*h)
    }

    fn cofactor(&self, row: usize, col: usize) -> f32 {
        let f = if (row+col) % 2 == 0 { 1.0_f32 } else { -1.0_f32 };
        f * self.minor(row,col)
    }

    pub fn determinant(&self) -> f32 {
          self.0[0] * self.minor(0,0) 
        - self.0[1] * self.minor(0,1) 
        + self.0[2] * self.minor(0,2) 
        - self.0[3] * self.minor(0,3)
    }

    pub fn invert(&self) -> M4 {
        let mut m = M4([ 0.0; 16 ]);
        let d = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                m.set(col, row, self.cofactor(row, col) / d)
            }
        }

        m
    }
}

pub fn mmul(a: &M4, b: &M4) -> M4 {
    let mut c = M4([ 0.0; 16 ]);

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

pub fn mvmul(m: &M4, v: &V4) -> V4 {
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

use float_cmp::ApproxEq;

impl ApproxEq for V4 {
    type Margin = float_cmp::F32Margin;

    fn approx_eq<T: Into<float_cmp::F32Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();

        self.0.approx_eq(other.0, margin) &&
        self.1.approx_eq(other.1, margin) &&
        self.2.approx_eq(other.2, margin) &&
        self.3.approx_eq(other.3, margin)
    }
}

impl<'a> ApproxEq for &'a M4 {
    type Margin = float_cmp::F32Margin;

    fn approx_eq<T: Into<float_cmp::F32Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();

        for row in 0..4 {
            for col in 0..4 {
                if !self.0[row*4+col].approx_eq(other.0[row*4+col], margin) {
                    return false;
                }
            }
        }

        true
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use float_cmp::*;

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
        let mut a = M4::identity();
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
        let a = M4([ 
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]);
        let b = M4([
            -2.0, 1.0, 2.0,  3.0,
             3.0, 2.0, 1.0, -1.0,
             4.0, 3.0, 6.0,  5.0,
             1.0, 2.0, 7.0,  8.0
        ]);

        let res = M4([
            20.0, 22.0,  50.0,  48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0,  46.0,  42.0
        ]);

        assert_eq!(mmul(&a, &b), res);

        let  i = M4::identity();
        assert_eq!(mmul(&a, &i), a);
    }

    #[test]
    fn mv_mul() {
        let a = M4([
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
        let m = M4([
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0
        ]);
        let t = M4([
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0
        ]);

        assert_eq!(m.transpose(), t);
        assert_eq!(M4::identity().transpose(), M4::identity());
    }

    #[test]
    fn determinant() {
        let m = M4([
            -2.0, -8.0,  3.0,  5.0,
            -3.0,  1.0,  7.0,  3.0,
             1.0,  2.0, -9.0,  6.0,
            -6.0,  7.0,  7.0, -9.0
        ]);

        assert_eq!(m.cofactor(0, 0),   690.0);
        assert_eq!(m.cofactor(0, 1),   447.0);
        assert_eq!(m.cofactor(0, 2),   210.0);
        assert_eq!(m.cofactor(0, 3),    51.0);
        assert_eq!(m.determinant(),  -4071.0);
    }

    #[test]
    fn invert() {
        let m = M4([
             9.0,   3.0,  0.0,  9.0,
            -5.0,  -2.0, -6.0, -3.0,
            -4.0,   9.0,  6.0,  4.0,
            -7.0,   6.0,  6.0,  2.0
        ]).invert();

        let result = M4([
            -0.04074, -0.07778,  0.14444, -0.22222,
            -0.07778,  0.03333,  0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926,  0.12963,
             0.17778,  0.06667, -0.26667,  0.33333
        ]);

        // for r in 0..4 {
        //     for c in 0..4 {
        //         assert!(approx_eq!(f32, m.at(r,c), result.at(r,c), epsilon = 0.0001), "m[{},{}] = {} (expected {})", r,c,m.at(r,c),result.at(r,c))
        //     }
        // }

        assert!(approx_eq!(&M4, &m, &result, epsilon = 0.0001));
    }
}