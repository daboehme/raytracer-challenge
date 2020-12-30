use crate::canvas::Canvas;
use crate::color::Color;
use crate::linalg;
use crate::linalg::{V4,M4};
use crate::ray::Ray;
use crate::world::World;

pub struct Camera {
    width: usize,
    height: usize,
    fov:   f32,
    transform_i: M4,
    half_width: f32,
    half_height: f32,
    pixel_size: f32
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f32, transform: &M4) -> Camera {
        let hv = (fov / 2.0).tan();
        let aspect = (hsize as f32) / (vsize as f32);

        let mut halfw = hv;
        let mut halfh = hv / aspect;

        if aspect < 1.0 {
            halfw = hv * aspect;
            halfh = hv
        }

        Camera {
            width: hsize,
            height: vsize,
            fov: fov,
            transform_i: transform.invert(),
            half_width: halfw,
            half_height: halfh,
            pixel_size: (halfw * 2.0) / (hsize as f32)
        }
    }

    pub fn new_default(hsize: usize, vsize: usize) -> Camera {
        let trans = M4::identity();

        Camera::new(hsize, vsize, std::f32::consts::FRAC_PI_2, &trans)
    }

    fn ray(&self, x: usize, y: usize) -> Ray {
        let xoff = ((x as f32) + 0.5) * self.pixel_size;
        let yoff = ((y as f32) + 0.5) * self.pixel_size;

        let wx = self.half_width - xoff;
        let wy = self.half_height - yoff;

        let pxp = linalg::mvmul(&self.transform_i, &V4::make_point(wx, wy, -1.0));
        let origin = linalg::mvmul(&self.transform_i, &V4::make_point(0.0, 0.0, 0.0));

        Ray {
            origin: origin,
            direction: (pxp - origin).normalize()
        }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height, Color::BLACK);

        for y in 0..self.height {
            for x in 0..self.width {
                let ray = self.ray(x, y);
                canvas.set(x, y, world.color_at(&ray));
            }
        }

        canvas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::*;
    use crate::transform::Transform;

    use float_cmp::*;

    #[test]
    fn pixelsize() {
        let c = Camera::new_default(200, 125);
        assert_eq!(c.pixel_size, 0.01);

        let c = Camera::new_default(125, 200);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn ray_center() {
        let c = Camera::new_default(201, 101);
        let r = c.ray(100, 50);

        assert!(approx_eq!(V4, r.origin, V4::make_point(0.0, 0.0, 0.0)));
        assert!(approx_eq!(V4, r.direction, V4::make_vector(0.0, 0.0, -1.0)));
    }

    #[test]
    fn ray_corner() {
        let c = Camera::new_default(201, 101);
        let r = c.ray(0, 0);

        assert!(approx_eq!(V4, r.origin, V4::make_point(0.0, 0.0, 0.0)));
        assert!(approx_eq!(V4, r.direction, V4::make_vector(0.66519, 0.33259, -0.66851), epsilon = 0.0001));
    }

    #[test]
    fn ray_trans() {
        let t = Transform::new()
            .rotate_y(std::f32::consts::FRAC_PI_4)
            .translate(0.0, -2.0, 5.0);

        let c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2, &t.matrix);
        let r = c.ray(100, 50);

        let sq2half = 0.5 * std::f32::consts::SQRT_2;

        assert!(approx_eq!(V4, r.origin, V4::make_point(0.0, 2.0, -5.0)));
        assert!(approx_eq!(V4, r.direction, V4::make_vector(sq2half, 0.0, -sq2half), epsilon = 0.0001));
    }
}