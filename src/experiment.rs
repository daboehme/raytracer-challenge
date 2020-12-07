use crate::canvas;
use crate::linalg;
use crate::objects;
use crate::ray;
use crate::render;

use objects::SceneObject;

pub fn draw_sphere() -> canvas::Canvas {
    let origin = linalg::V4::make_point(0.0, 0.0, -5.0);

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

            let pos = linalg::V4::make_point(world_x, world_y, world_z);
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