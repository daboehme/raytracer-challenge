use crate::color::Color;
use crate::lighting::LightSource;
use crate::lighting;
use crate::linalg::V4;
use crate::ray::Ray;
use crate::shape::Shape;

use std::rc::Rc;

pub struct World {
    lights: Vec<LightSource>,
    shapes: Vec< Rc<Shape> >,

    max_recursion: u32
}

type Intersection = (f32, Rc<Shape>);

fn hit(xs: &[Intersection]) -> Option<&Intersection> {
    xs.iter().find(|&x| x.0 >= 0.0)
}

impl World {
    pub fn new() -> World {
        World {
            lights: vec![],
            shapes: vec![],
            max_recursion: 5
        }
    }

    pub fn new_with(lights: Vec<LightSource>, shapes: Vec<Rc<Shape>>) -> World {
        World {
            lights: lights,
            shapes: shapes,
            max_recursion: 5
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

    fn shade(&self, ray: &Ray, hit: f32, obj: &Shape, recurse: u32) -> V4 {
        let point = ray.position(hit);
        let eyev  = -ray.direction;
        let mut normalv = obj.normal_at(point);

        if V4::dot(&normalv, &eyev) < 0.0 {
            normalv = -normalv
        }

        // push point in direction of normal to avoid peppering
        let point = point + normalv * 0.0001;

        let material = obj.material();

        let mut colorv = V4::from(Color::BLACK);

        for light in self.lights.iter() {
            colorv +=
                lighting::lighting(
                    material,
                    light,
                    &point,
                    &eyev,
                    &normalv,
                    self.is_shadowed(light, &point)
                );
        }

        if recurse > 0 && material.reflective > 0.0 {
            let rfl_ray = Ray::new(point, V4::reflect(ray.direction, normalv));
            let rfl_clr = self.recursive_color_at(&rfl_ray, recurse-1);

            colorv += rfl_clr * material.reflective;
        }

        colorv
    }

    fn recursive_color_at(&self, ray: &Ray, recurse: u32) -> V4 {
        match hit(self.intersections(ray).as_slice()) {
            Some((dst, obj)) => self.shade(ray, *dst, obj, recurse),
            None => V4::from(Color::BLACK)
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        Color::from(self.recursive_color_at(ray, self.max_recursion))
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
    use crate::plane::Plane;
    use crate::transform::Transform;

    use super::*;
    use float_cmp::*;

    const MATERIAL : Material = Material {
        texture: Texture::Color(Color::WHITE),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.0
    };

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
            shininess: 200.0,
            reflective: 0.0
        };

        w.shapes.push(Rc::new(Shape::new(Box::new(Sphere()), &m, &t.matrix)));

        let t = Transform::new().scale(0.5, 0.5, 0.5);
        let m = Material {
            texture: Texture::Color(Color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0
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
        let r = Ray::new(V4::new_point(0.0, 0.0, -5.0), V4::new_vector(0.0, 0.0, 1.0));

        let c = Color::from(w.shade(&r, 4.0, &w.shapes[0], 1));

        assert!(approx_eq!(f32, c.r, 0.38066, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.g, 0.47583, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.b, 0.2855,  epsilon = 0.0001));
    }

    #[test]
    fn shade_ins() {
        let mut w = make_world();
        w.lights[0].pos = V4::new_point(0.0, 0.25, 0.0);

        let r = Ray::new(V4::new_point(0.0, 0.0, 0.0), V4::new_vector(0.0, 0.0, 1.0));
        let c = Color::from(w.shade(&r, 0.5, &w.shapes[1], 1));

        assert!(approx_eq!(f32, c.r, 0.90498, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.g, 0.90498, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.b, 0.90498, epsilon = 0.0001));
    }

    #[test]
    fn color_miss() {
        let w = make_world();
        let r = Ray::new(V4::new_point(0.0, 0.0, -5.0), V4::new_vector(0.0, 1.0, 0.0));

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

    #[test]
    fn shade_with_reflection() {
        let mut w = make_world();

        let mut m = MATERIAL;
        m.reflective = 0.5;

        let t = Transform::new().translate(0.0, -1.0, 0.0);
        let s = Rc::new(Shape::new(Box::new(Plane()), &m, &t.matrix));
        w.add_shape(Rc::clone(&s));

        let sqrth = std::f32::consts::SQRT_2 * 0.5;

        let r = Ray::new(V4::new_point(0.0, 0.0, -3.0), V4::new_vector(0.0, -sqrth, sqrth));

        let c = w.shade(&r, std::f32::consts::SQRT_2, &s, 4);

        assert!(approx_eq!(V4, c, V4::new_vector(0.87677, 0.92436, 0.82918), epsilon = 0.0001));
    }

    #[test]
    fn recursion_limit() {
        let mut m = MATERIAL;
        m.reflective = 1.0;

        let tl = Transform::new().translate(0.0, -1.0, 0.0);
        let tu = Transform::new().translate(0.0,  1.0, 0.0);

        let l = LightSource {
            pos: V4::new_point(0.0, 0.0, 0.0),
            intensity: Color::WHITE
        };

        let shapes = vec![
            Rc::new(Shape::new(Box::new(Plane()), &m, &tl.matrix)),
            Rc::new(Shape::new(Box::new(Plane()), &m, &tu.matrix))
        ];

        let w = World::new_with(vec![l], shapes);
        let r = Ray::new(V4::new_point(0.0, 0.0, 0.0), V4::new_vector(0.0, 1.0, 0.0));

        // should not infinitely recurse
        let c = w.color_at(&r);

        assert_ne!(c, Color::BLACK);
    }
}
