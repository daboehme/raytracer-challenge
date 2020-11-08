#[derive(Clone,Copy,Debug)]
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

    pub fn add(a: &V4, b: &V4) -> V4 {
        V4(a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
    }

    pub fn sub(a: &V4, b: &V4) -> V4 {
        V4(a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
    }

    pub fn mult(f: f32, a: &V4) -> V4 {
        V4(f * a.0, f * a.1, f * a.2, f * a.3)
    }

    pub fn dot(a: &V4, b: &V4) -> f32 {
        a.0*b.0 + a.1*b.1 + a.2*b.2 + a.3*b.3
    }

    pub fn cross(a: &V4, b: &V4) -> V4 {
        V4::make_vector(a.1*b.2-a.2*b.1, a.2*b.0-a.0*b.2, a.0*b.1-a.1*b.0)
    }
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
}