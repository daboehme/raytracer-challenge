use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::linalg::{M4,V4};
use crate::lighting;
use crate::lighting::{Material,LightSource};
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::ray;
use crate::transform::Transform;
use crate::world::World;

use std::rc::Rc;

const DEFAULT_MATERIAL: Material = Material {
    color: Color { r: 1.0, g: 0.2, b: 1.0 },
    ambient: 0.1,
    diffuse: 0.7,
    specular: 0.2,
    shininess: 200.0
};

pub fn draw_sphere() -> Canvas {
    let origin = V4::make_point(0.0, 0.0, -5.0);

    let canvas_px = 100;
    let wall_size = 7.0;
    let world_z = 10.0;
    let px_size = wall_size / (canvas_px as f32);

    let mut canvas = Canvas::new(canvas_px, canvas_px, Color::BLACK);

    let s = Shape::new(Box::new(Sphere()), &DEFAULT_MATERIAL, &M4::identity());

    for y in 0..canvas_px {
        let world_y = 0.5 * wall_size - px_size * (y as f32);

        for x in 0..canvas_px {
            let world_x = -0.5 * wall_size + px_size * (x as f32);

            let pos = V4::make_point(world_x, world_y, world_z);
            let r = ray::Ray {
                origin: origin, direction: (pos - origin).normalize()
            };

            if !s.intersect(&r).is_empty() {
                canvas.set(x, y, Color::RED);
            }
        }
    }

    canvas
}

pub fn draw_sphere_lighting() -> Canvas {
    let light = LightSource {
        intensity: Color { r: 1.0, g: 1.0, b: 1.0 },
        pos: V4::make_point(-10.0, 10.0, -10.0)
    };

    let origin = V4::make_point(0.0, 0.0, -5.0);

    let canvas_px = 256;
    let wall_size = 7.0;
    let world_z = 10.0;
    let px_size = wall_size / (canvas_px as f32);

    let mut canvas = Canvas::new(canvas_px, canvas_px, Color::BLACK);

    let s = Shape::new(Box::new(Sphere()), &DEFAULT_MATERIAL, &M4::identity());

    for y in 0..canvas_px {
        let world_y = 0.5 * wall_size - px_size * (y as f32);

        for x in 0..canvas_px {
            let world_x = -0.5 * wall_size + px_size * (x as f32);

            let pos = V4::make_point(world_x, world_y, world_z);
            let ray = ray::Ray {
                origin: origin, direction: (pos - origin).normalize()
            };

            let xs = s.intersect(&ray);

            if !xs.is_empty() {
                let m = s.material();
                let p = ray.position(xs[0]);
                let nrm = s.normal_at(p);
                let eye = -ray.direction;

                canvas.set(x, y, Color::from(lighting::lighting(&m, &light, &p, &eye, &nrm, false)));
            }
        }
    }

    canvas
}

pub fn draw_world() -> Canvas {
    let mut world = World::new();

    world.add_light( &LightSource {
            pos: V4::make_point(-10.0, 10.0, -10.0),
            intensity: Color::WHITE
        } );

    let m = Material {
        color: Color::new(0.1, 1.0, 0.5),
        ambient: 0.1,
        diffuse: 0.7,
        specular: 0.3,
        shininess: 200.0
    };

    let t = Transform::new().translate(-0.5, 1.0, 0.5);

    world.add_object(Rc::new(Shape::new(Box::new(Sphere()), &m, &t.matrix)));

    let t = Transform::new()
        .translate(1.5, 0.5, -0.5)
        .scale(0.5, 0.5, 0.5);

    world.add_object(Rc::new(Shape::new(Box::new(Sphere()), &m, &t.matrix)));

    let t = Transform::new()
        .translate(-1.5, 0.33, -0.75)
        .scale(0.33, 0.33, 0.33);

    world.add_object(Rc::new(Shape::new(Box::new(Sphere()), &m, &t.matrix)));

    let from = V4::make_point(0.0, 1.5, -5.0);
    let to = V4::make_point(0.0, 1.0, 0.0);
    let up = V4::make_vector(0.0, 1.0, 0.0);

    let vt = Transform::view_transform(&from, &to, &up);

    Camera::new(480, 320, std::f32::consts::FRAC_PI_3, &vt.matrix).render(&world)
}