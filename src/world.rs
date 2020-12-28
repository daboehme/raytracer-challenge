use crate::color::Color;
use crate::objects::Object;
use crate::ray::Ray;
use crate::lighting::LightSource;
use crate::lighting;
use crate::linalg::V4;

use std::rc::Rc;

pub struct World {
    lights: Vec<LightSource>,
    objects: Vec<Rc<dyn Object>>
}

impl World {
    pub fn new() -> World {
        World {
            lights: vec![],
            objects: vec![]
        }
    }

    fn intersections(&self, ray: &Ray) -> Vec< (f32, Rc<dyn Object>) > {
        let mut result = Vec::new();

        for object in self.objects.iter() {
            let hits = object.intersect(ray);

            for hit in hits {
                result.push( (hit, Rc::clone(&object)) )
            }
        }

        result.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        result
    }

    fn shade(&self, ray: &Ray, hit: f32, obj: Rc<dyn Object>) -> Color {
        let point = ray.position(hit);
        let eyev  = -ray.direction;
        let mat   = obj.material();
        let mut normalv = obj.normal_at(point);

        if V4::dot(&normalv, &eyev) < 0.0 {
            normalv = -normalv
        }

        let mut colorv = V4::from(Color::BLACK);

        for light in self.lights.iter() {
            colorv += lighting::lighting(&mat, light, &point, &eyev, &normalv)
        }

        Color::from(colorv)
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersections(&ray);

        match xs.iter().find(|&x| x.0 >= 0.0) {
            Some((dst, obj)) => self.shade(ray, *dst, Rc::clone(obj)),
            None => Color::BLACK
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::linalg::V4;
    use crate::objects::Sphere;
    use crate::lighting::*;
    use crate::transform::Transform;

    use super::*;
    use float_cmp::*;

    fn make_world() -> World {
        let mut w = World::new();

        w.lights.push( LightSource {
                intensity: Color::WHITE,
                pos: V4::make_point(-10.0, 10.0, -10.0)
            } );
        
        let t = Transform::new();
        let m = Material {
            color: Color::new(0.8, 1.0, 0.6),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0
        };

        w.objects.push(Rc::new(Sphere::new_custom(&m, &t.matrix)));

        let t = Transform::new().scale(0.5, 0.5, 0.5);
        let m = Material {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0    
        };

        w.objects.push(Rc::new(Sphere::new_custom(&m, &t.matrix)));

        w
    }

    #[test]
    fn intersections() {
        let w = make_world();
        let r = Ray { 
            origin: V4::make_point(0.0, 0.0, -5.0), 
            direction: V4::make_vector(0.0, 0.0, 1.0)
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
            origin: V4::make_point(0.0, 0.0, -5.0),
            direction: V4::make_vector(0.0, 0.0, 1.0)
        };

        let c = w.shade(&r, 4.0, Rc::clone(&w.objects[0]));
 
        assert!(approx_eq!(f32, c.r, 0.38066, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.g, 0.47583, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.b, 0.2855,  epsilon = 0.0001));
    }

    #[test]
    fn shade_ins() {
        let mut w = make_world();
        w.lights[0].pos = V4::make_point(0.0, 0.25, 0.0);

        let r = Ray {
            origin: V4::make_point(0.0, 0.0, 0.0),
            direction: V4::make_vector(0.0, 0.0, 1.0)
        };

        let xs = w.intersections(&r);

        let c = w.shade(&r, 0.5, Rc::clone(&w.objects[1]));

        assert!(approx_eq!(f32, c.r, 0.90498, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.g, 0.90498, epsilon = 0.0001));
        assert!(approx_eq!(f32, c.b, 0.90498, epsilon = 0.0001));
    }

    #[test]
    fn color_miss() {
        let w = make_world();
        let r = Ray {
            origin: V4::make_point(0.0, 0.0, -5.0),
            direction: V4::make_vector(0.0, 1.0, 0.0)
        };

        assert_eq!(w.color_at(&r), Color::BLACK);
    }

    #[test]
    fn color_hit() {
        let w = make_world();
        let r = Ray {
            origin: V4::make_point(0.0, 0.0, -5.0),
            direction: V4::make_vector(0.0, 0.0, 1.0)
        };

        let c = w.color_at(&r);

        assert!(approx_eq!(V4, V4::from(c), V4::make_vector(0.38066, 0.47583, 0.2855), epsilon = 0.0001));
    }
}