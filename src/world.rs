use crate::color::Color;
use crate::lighting::LightSource;
use crate::lighting;
use crate::linalg::V4;
use crate::ray::Ray;
use crate::shape::Shape;

use std::rc::Rc;

pub struct World {
    lights: Vec<LightSource>,
    shapes: Vec< Rc<Shape> >
}

type Intersection = (f32, Rc<Shape>);

fn hit(xs: &[Intersection]) -> Option<&Intersection> {
    xs.iter().find(|&x| x.0 >= 0.0)
}

impl World {
    pub fn new() -> World {
        World {
            lights: vec![],
            shapes: vec![]
        }
    }

    pub fn new_with(lights: Vec<LightSource>, shapes: Vec<Rc<Shape>>) -> World {
        World {
            lights: lights,
            shapes: shapes
        }
    }

    fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs = Vec::new();

        for shape in self.shapes.iter() {
            for t in shape.intersect(ray) {
                xs.push( (t, Rc::clone(&shape)) )
            }
        }

        xs.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        xs
    }

    fn is_shadowed(&self, light: &LightSource, point: &V4) -> bool {
        let v = light.pos - *point;
        let r = Ray {
            origin: *point,
            direction: v.normalize()
        };

        match hit(self.intersections(&r).as_slice()) {
            Some((dst, _)) => dst < &v.magnitude(),
            None => false
        }
    }

    fn shade(&self, ray: &Ray, hit: f32, obj: Rc<Shape>) -> Color {
        let point = ray.position(hit);
        let eyev  = -ray.direction;
        let mut normalv = obj.normal_at(point);

        if V4::dot(&normalv, &eyev) < 0.0 {
            normalv = -normalv
        }

        let over_point = point + normalv * 0.001;

        let mut colorv = V4::from(Color::BLACK);

        for light in self.lights.iter() {
            colorv +=
                lighting::lighting(
                    obj.material(),
                    light,
                    &point,
                    &eyev,
                    &normalv,
                    self.is_shadowed(light, &over_point)
                );
        }

        Color::from(colorv)
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        match hit(self.intersections(ray).as_slice()) {
            Some((dst, obj)) => self.shade(ray, *dst, Rc::clone(&obj)),
            None => Color::BLACK
        }
    }

    pub fn add_shape(&mut self, obj: Rc<Shape>) {
        self.shapes.push(Rc::clone(&obj));
    }

    pub fn add_light(&mut self, light: &LightSource) {
        self.lights.push(*light);
    }
}


#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::linalg::V4;
    use crate::material::{Material,Texture};
    use crate::sphere::Sphere;
    use crate::lighting::*;
    use crate::transform::Transform;

    use super::*;
    use float_cmp::*;

    fn make_world() -> World {
        let mut w = World::new();

        w.lights.push( LightSource {
                intensity: Color::WHITE,
                pos: V4::new_point(-10.0, 10.0, -10.0)
            } );

        let t = Transform::new();
        let m = Material {
            texture: Texture::Color(Color::new(0.8, 1.0, 0.6)),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0
        };

        w.shapes.push(Rc::new(Shape::new(Box::new(Sphere()), &m, &t.matrix)));

        let t = Transform::new().scale(0.5, 0.5, 0.5);
        let m = Material {
            texture: Texture::Color(Color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        };

        w.shapes.push(Rc::new(Shape::new(Box::new(Sphere()), &m, &t.matrix)));

        w
    }

    #[test]
    fn intersections() {
        let w = make_world();
        let r = Ray {
            origin: V4::new_point(0.0, 0.0, -5.0),
            direction: V4::new_vector(0.0, 0.0, 1.0)
        };

        let xs = w.intersections(&r);

        assert_eq!(xs.len(), 4);

        assert_eq!(xs[0].0, 4.0);
        assert_eq!(xs[1].0, 4.5);
        assert_eq!(xs[2].0, 5.5);
        assert_eq!(xs[3].0, 6.0)
    }

    #[test]
    fn shade_hit() {
        let w = make_world();
        let r = Ray {
            origin: V4::new_point(0.0, 0.0, -5.0),
            direction: V4::new_vector(0.0, 0.0, 1.0)
        };

        let c = w.shade(&r, 4.0, Rc::clone(&w.shapes[0]));

        assert!(approx_eq!(f32, c.r, 0.38066, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.g, 0.47583, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.b, 0.2855,  epsilon = 0.0001));
    }

    #[test]
    fn shade_ins() {
        let mut w = make_world();
        w.lights[0].pos = V4::new_point(0.0, 0.25, 0.0);

        let r = Ray {
            origin: V4::new_point(0.0, 0.0, 0.0),
            direction: V4::new_vector(0.0, 0.0, 1.0)
        };

        let c = w.shade(&r, 0.5, Rc::clone(&w.shapes[1]));

        assert!(approx_eq!(f32, c.r, 0.90498, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.g, 0.90498, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.b, 0.90498, epsilon = 0.0001));
    }

    #[test]
    fn color_miss() {
        let w = make_world();
        let r = Ray {
            origin: V4::new_point(0.0, 0.0, -5.0),
            direction: V4::new_vector(0.0, 1.0, 0.0)
        };

        assert_eq!(w.color_at(&r), Color::BLACK);
    }

    #[test]
    fn color_hit() {
        let w = make_world();
        let r = Ray {
            origin: V4::new_point(0.0, 0.0, -5.0),
            direction: V4::new_vector(0.0, 0.0, 1.0)
        };

        let c = w.color_at(&r);

        assert!(approx_eq!(V4, V4::from(c), V4::new_vector(0.38066, 0.47583, 0.2855), epsilon = 0.0001));
    }

    #[test]
    fn render() {
        let w = make_world();

        let from = V4::new_point(0.0, 0.0, -5.0);
        let to = V4::new_point(0.0, 0.0, 0.0);
        let up = V4::new_vector(0.0, 1.0, 0.0);

        let t = Transform::view_transform(&from, &to, &up);
        let c = Camera::new(11, 11, std::f32::consts::FRAC_PI_2, &t.matrix);

        let v = c.render_to_canvas(&w).at(5, 5);

        assert!(approx_eq!(f32, v.r, 0.38066, epsilon = 0.0001));
        assert!(approx_eq!(f32, v.g, 0.47583, epsilon = 0.0001));
        assert!(approx_eq!(f32, v.b, 0.2855, epsilon = 0.0001));
    }

    #[test]
    fn shadow() {
        let w = make_world();
        let p = V4::new_point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(&w.lights.first().unwrap(), &p));

        let p = V4::new_point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&w.lights.first().unwrap(), &p));

        let p = V4::new_point(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&w.lights.first().unwrap(), &p));
    }
}