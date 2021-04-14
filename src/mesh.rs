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

use std::io;
use std::io::prelude::*;

fn parse_obj(reader: &mut dyn io::BufRead) -> io::Result< Vec<Triangle> > {
    let mut result = Vec::new();
    let mut vertices = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        match split.next() {
            Some("v") => {
                let v: Vec<f32> = split.map(|s| s.parse::<f32>().unwrap()).collect();
                if v.len() >= 3 {
                    vertices.push( (v[0], v[1], v[2]) );
                }
                ()
            }
            Some("f") => {
                let indices: Vec<i32> = split.map(|s| s.parse::<i32>().unwrap()).collect();
                if indices.len() >= 3 {
                    for index in 2..indices.len() {
                        let v  = vertices[(indices[0      ] - 1) as usize];
                        let p0 = V4::new_vector(v.0, v.1, v.2);
                        let v  = vertices[(indices[index-1] - 1) as usize];
                        let p1 = V4::new_vector(v.0, v.1, v.2);
                        let v  = vertices[(indices[index  ] - 1) as usize];
                        let p2 = V4::new_vector(v.0, v.1, v.2);
                        result.push(Triangle::new(p0, p1, p2))
                    }
                }
            }
            _ => ()
        }
    }

    Ok(result)
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

    #[test]
    fn parse_triangles() {
        let mut input = "
            v -1 1 0
            v -1 0 0
            v 1 0 0 
            v 1 1 0
            f 1 2 3
            f 1 3 4
        ".as_bytes();

        let triangles = parse_obj(&mut input).unwrap();

        assert_eq!(triangles.len(), 2);
        assert_eq!(triangles[0].p[0], V4::new_vector(-1.0, 1.0, 0.0));
        assert_eq!(triangles[0].p[1], V4::new_vector(-1.0, 0.0, 0.0));
        assert_eq!(triangles[0].p[2], V4::new_vector( 1.0, 0.0, 0.0));

        assert_eq!(triangles[1].p[0], V4::new_vector(-1.0, 1.0, 0.0));
        assert_eq!(triangles[1].p[1], V4::new_vector( 1.0, 0.0, 0.0));
        assert_eq!(triangles[1].p[2], V4::new_vector( 1.0, 1.0, 0.0));
    }
}
