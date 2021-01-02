use crate::linalg::V4;
use crate::ray::Ray;
use crate::shape::BaseShape;

use std::mem;

pub struct WavyPlane {
    origin: V4,
    wavelength: f32,
    amplitude: f32
}

impl WavyPlane {
    pub fn new (origin: &V4, len: f32, amp: f32) -> WavyPlane {
        WavyPlane {
            origin: *origin,
            wavelength: len,
            amplitude: amp
        }
    }

    fn eval(&self, point: V4) -> f32 {
        let p = V4::make_point(point.x(), 0.0, point.z());
        let x = (p - self.origin).magnitude();
        self.amplitude * (self.wavelength * x).sin()
    }
}

impl BaseShape for WavyPlane {
    fn intersect(&self, r: &Ray) -> Vec<f32> {
        let is_in_wavezone = r.origin.y().abs() <= self.amplitude;

        if r.direction.y().abs() < 0.001 && !is_in_wavezone {
            return vec![]
        }

        if r.direction.x().abs() + r.direction.z().abs() < 0.01 {
            // looking directly from above/below
            return vec![ self.eval(r.origin) - r.origin.y() ]
        }

        // find bounds

        let mut dsta = if is_in_wavezone {
                -std::f32::consts::PI
            } else {
                -(r.origin.y() + self.amplitude) / r.direction.y()
            };
        let mut dstb = if is_in_wavezone {
                 std::f32::consts::PI
            } else {
                -(r.origin.y() - self.amplitude) / r.direction.y()
            };

        if dstb < dsta { mem::swap(&mut dsta, &mut dstb); }

        let p0 = r.origin + r.direction.normalize() * dsta;
        let p1 = r.origin + r.direction.normalize() * dstb;

        const STEP : f32 = 0.1;

        let d = (p1 - p0).normalize() * STEP;
        let len = dstb-dsta;
        let mut x = 0.0;
        let mut pa = p0;
        let mut sa = self.eval(pa);

        let mut results = Vec::new();

        while x < len {
            let pb = pa + d;
            let mut sb = self.eval(pb);

            let mut ra = pa.y();
            let mut rb = pb.y();

            if sb < sa { mem::swap(&mut sa, &mut sb); }
            if rb < ra { mem::swap(&mut ra, &mut rb); }

            //   crude approximation: if there is overlap,
            // count center of segment as intersection
            if sa <= rb && ra <= sb {
                results.push(dsta + x * 0.5*STEP)
            }

            pa = pb;
            sa = sb;
            x += STEP
        }

        results
    }

    fn normal_at(&self, p: V4) -> V4 {
        const EPS : f32 = 0.01;

        let xa = p + V4::make_vector(-EPS, 0.0, 0.0);
        let xb = p + V4::make_vector( EPS, 0.0, 0.0);

        let dx = (self.eval(xb) - self.eval(xa)) / (2.0 * EPS);

        let za = p + V4::make_vector(0.0, 0.0, -EPS);
        let zb = p + V4::make_vector(0.0, 0.0,  EPS);

        let dz = (self.eval(zb) - self.eval(za)) / (2.0 * EPS);

        V4::make_vector(-dx, 1.0, -dz).normalize()
    }
}
