use crate::canvas;
use crate::linalg::V4;
use crate::objects;
use crate::ray;
use crate::render;

use objects::SceneObject;

pub fn draw_sphere() -> canvas::Canvas {
    let origin = V4::make_point(0.0, 0.0, -5.0);

    let canvas_px = 100;
    let wall_size = 7.0;
    let world_z = 10.0;
    let px_size = wall_size / (canvas_px as f32);

    let mut canvas = canvas::Canvas::new(canvas_px, canvas_px, render::Color::BLACK);

    let s = objects::Sphere::new();

    for y in 0..canvas_px {
        let world_y = 0.5 * wall_size - px_size * (y as f32);

        for x in 0..canvas_px {
            let world_x = -0.5 * wall_size + px_size * (x as f32);

            let pos = V4::make_point(world_x, world_y, world_z);
            let r = ray::Ray {
                origin: origin, direction: (pos - origin).normalize()
            };

            if !s.intersect(&r).is_empty() {
                canvas.set(x, y, render::Color::RED);
            }
        }
    }

    canvas
}

pub fn draw_sphere_lighting() -> canvas::Canvas {
    let light = render::LightSource {
        intensity: render::Color { r: 1.0, g: 1.0, b: 1.0 },
        pos: V4::make_point(-10.0, 10.0, -10.0)
    };

    let origin = V4::make_point(0.0, 0.0, -5.0);

    let canvas_px = 256;
    let wall_size = 7.0;
    let world_z = 10.0;
    let px_size = wall_size / (canvas_px as f32);

    let mut canvas = canvas::Canvas::new(canvas_px, canvas_px, render::Color::BLACK);

    let s = objects::Sphere::new();
 
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

                canvas.set(x, y, render::lighting(&m, &light, &p, &eye, &nrm));
            }
        }
    }

    canvas
}
