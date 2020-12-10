use crate::linalg::V4;

use std::convert::From;

#[derive(Clone,Copy,Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };
    pub const RED:   Color = Color { r: 1.0, g: 0.0, b: 0.0 };
}

impl From<V4> for Color {
    fn from(v: V4) -> Color {
        Color { r: v.x(), g: v.y(), b: v.z() }
    }
}

impl From<Color> for V4 {
    fn from(c: Color) -> V4 {
        V4::make_vector(c.r, c.g, c.b)
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32
}

#[derive(Clone,Copy,Debug)]
pub struct LightSource {
    pub intensity: Color,
    pub pos: V4
}

pub fn lighting
    (
        material: &Material,
        light:    &LightSource,
        point:    &V4,
        eyev:     &V4,
        normalv:  &V4
    ) -> Color
{
    let mc = material.color;
    let lc = light.intensity;
    let colorv = V4::make_vector(mc.r*lc.r, mc.g*lc.g, mc.b*lc.b);
    let lightv = (light.pos - *point).normalize();

    let ambient = colorv * material.ambient;

    let mut diffuse  = V4::from(Color::BLACK);
    let mut specular = V4::from(Color::BLACK);

    let light_dot_normal = V4::dot(&lightv, normalv);

    if light_dot_normal >= 0.0 {
        diffuse = colorv * material.diffuse * light_dot_normal;

        let reflectv = V4::reflect(-lightv, *normalv);
        let reflect_dot_eye = V4::dot(&reflectv, eyev);

        if reflect_dot_eye > 0.0 {
            let f = reflect_dot_eye.powf(material.shininess);
            specular = V4::from(light.intensity) * (f * material.specular);
        }
    }

    Color::from(ambient + diffuse + specular)
}


#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::*;

    const MATERIAL : Material = Material {
        color: Color::WHITE,
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0
    };

    #[test]
    fn frontal_lighting() {
        let eyev = V4::make_vector(0.0, 0.0, -1.0);
        let normalv = V4::make_vector(0.0, 0.0, -1.0);
        let light = LightSource {
            intensity: Color::WHITE,
            pos: V4::make_point(0.0, 0.0, -10.0)
        };
        let pos = V4::make_point(0.0, 0.0, 0.0);

        let val = lighting(&MATERIAL, &light, &pos, &eyev, &normalv);
        let val = V4::from(val);

        assert!(approx_eq!(V4, val, V4::make_vector(1.9, 1.9, 1.9), epsilon = 0.0001));
    }

    #[test]
    fn angled_lighting() {
        let sq2half = 0.5 * std::f32::consts::SQRT_2;
        let eyev = V4::make_vector(0.0, sq2half, sq2half);
        let normalv = V4::make_vector(0.0, 0.0, -1.0);
        let light = LightSource {
            intensity: Color::WHITE,
            pos: V4::make_point(0.0, 0.0, -10.0)
        };
        let pos = V4::make_point(0.0, 0.0, 0.0);

        let val = lighting(&MATERIAL, &light, &pos, &eyev, &normalv);
        let val = V4::from(val);

        assert!(approx_eq!(V4, val, V4::make_vector(1.0, 1.0, 1.0), epsilon = 0.0001));
    }

    #[test]
    fn opposite_surface() {
        let eyev = V4::make_vector(0.0, 0.0, -1.0);
        let normalv = V4::make_vector(0.0, 0.0, -1.0);
        let light = LightSource {
            intensity: Color::WHITE,
            pos: V4::make_point(0.0, 10.0, -10.0)
        };
        let pos = V4::make_point(0.0, 0.0, 0.0);

        let val = V4::from(lighting(&MATERIAL, &light, &pos, &eyev, &normalv));
        let exp = V4::make_vector(0.7364, 0.7364, 0.7364);

        assert!(approx_eq!(V4, val, exp, epsilon = 0.0001));
    }

    #[test]
    fn reflect_light() {
        let sq2half = 0.5 * std::f32::consts::SQRT_2;
        let eyev = V4::make_vector(0.0, -sq2half, -sq2half);
        let normalv = V4::make_vector(0.0, 0.0, -1.0);
        let light = LightSource {
            intensity: Color::WHITE,
            pos: V4::make_point(0.0, 10.0, -10.0)
        };
        let pos = V4::make_point(0.0, 0.0, 0.0);

        let val = V4::from(lighting(&MATERIAL, &light, &pos, &eyev, &normalv));
        let exp = V4::make_vector(1.6364, 1.6364, 1.6364);

        assert!(approx_eq!(V4, val, exp, epsilon = 0.0001));
    }
}
