use crate::linalg::V4;
use crate::ray::Ray;
use crate::shape::BaseShape;

#[derive(Clone,Copy,Debug,PartialEq)]
struct Triangle {
    pub p: [V4; 3],
    pub e: [V4; 2],
    pub normal: V4
}

impl Triangle {
    pub fn new(p1: V4, p2: V4, p3: V4) -> Triangle {
        let e1 = p2 - p1;
        let e2 = p3 - p1;

        Triangle {
            p: [ p1, p2, p3 ],
            e: [ e1, e2 ],
            normal: V4::cross(&e2, &e1).normalize()
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let dir_x_e2 = V4::cross(&ray.direction, &self.e[1]);
        let det = V4::dot(&self.e[0], &dir_x_e2);

        if det.abs() < 0.0001 {
            return None
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p[0];
        let u = f * V4::dot(&p1_to_origin, &dir_x_e2);

        if u < 0.0 || u > 1.0 {
            return None
        }
        
        let origin_x_e1 = V4::cross(&p1_to_origin, &self.e[0]);
        let v = f * V4::dot(&ray.direction, &origin_x_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return None
        }

        let t = f * V4::dot(&self.e[1], &origin_x_e1);

        Some(t)
    }
}



pub struct Mesh {

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::{Material,Texture};
    use crate::linalg::{M4,V4};
    use crate::transform::Transform;
    use crate::shape::Shape;
    use float_cmp::*;

    #[test]
    fn new_triangle() {
        let points = [ 
            V4::new_point( 0.0, 1.0, 0.0),
            V4::new_point(-1.0, 0.0, 0.0),
            V4::new_point( 1.0, 0.0, 0.0)
        ];
        let t = Triangle::new(points[0], points[1], points[2]);

        assert_eq!(t.e[0],   V4::new_vector(-1.0, -1.0,  0.0));
        assert_eq!(t.e[1],   V4::new_vector( 1.0, -1.0,  0.0));
        assert_eq!(t.normal, V4::new_vector( 0.0,  0.0, -1.0))
    }

    #[test]
    fn triangle_intersect() {
        let points = [ 
            V4::new_point( 0.0, 1.0, 0.0),
            V4::new_point(-1.0, 0.0, 0.0),
            V4::new_point( 1.0, 0.0, 0.0)
        ];
        let t = Triangle::new(points[0], points[1], points[2]);

        assert_eq!(t.intersect(&Ray::new(V4::new_point( 0.0, -1.0, -2.0), V4::new_vector(0.0, 1.0, 0.0))), None);
        assert_eq!(t.intersect(&Ray::new(V4::new_point( 1.0,  1.0, -2.0), V4::new_vector(0.0, 0.0, 1.0))), None);
        assert_eq!(t.intersect(&Ray::new(V4::new_point(-1.0,  1.0, -2.0), V4::new_vector(0.0, 0.0, 1.0))), None);
        assert_eq!(t.intersect(&Ray::new(V4::new_point( 0.0, -1.0, -2.0), V4::new_vector(0.0, 0.0, 1.0))), None);
        assert_eq!(t.intersect(&Ray::new(V4::new_point( 0.0,  0.5, -2.0), V4::new_vector(0.0, 0.0, 1.0))), Some(2.0));
    }
}
