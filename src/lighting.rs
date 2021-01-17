use crate::color::Color;
use crate::material::Material;
use crate::linalg::V4;

#[derive(Clone,Copy,Debug)]
pub struct LightSource {
    pub intensity: Color,
    pub pos: V4
}

pub fn lighting
    (
        material:  &Material,
        light:     &LightSource,
        point:     &V4,
        eyev:      &V4,
        normalv:   &V4,
        in_shadow: bool
    ) -> V4
{
    let mc = material.color_at(*point);
    let lc = light.intensity;
    let colorv = V4::new_vector(mc.r*lc.r, mc.g*lc.g, mc.b*lc.b);

    let ambient = colorv * material.ambient;

    let mut diffuse  = V4::from(Color::BLACK);
    let mut specular = V4::from(Color::BLACK);

    if !in_shadow {
        let lightv = (light.pos - *point).normalize();
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
    }

    ambient + diffuse + specular
}


#[cfg(test)]
mod tests {
    use crate::material::Texture;
    use super::*;
    use float_cmp::*;

    const MATERIAL : Material = Material {
        texture: Texture::Color(Color::WHITE),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0
    };

    #[test]
    fn frontal_lighting() {
        let eyev = V4::new_vector(0.0, 0.0, -1.0);
        let normalv = V4::new_vector(0.0, 0.0, -1.0);
        let light = LightSource {
            intensity: Color::WHITE,
            pos: V4::new_point(0.0, 0.0, -10.0)
        };
        let pos = V4::new_point(0.0, 0.0, 0.0);

        let val = lighting(&MATERIAL, &light, &pos, &eyev, &normalv, false);
        let val = V4::from(val);

        assert!(approx_eq!(V4, val, V4::new_vector(1.9, 1.9, 1.9), epsilon = 0.0001));
    }

    #[test]
    fn angled_lighting() {
        let sq2half = 0.5 * std::f32::consts::SQRT_2;
        let eyev = V4::new_vector(0.0, sq2half, sq2half);
        let normalv = V4::new_vector(0.0, 0.0, -1.0);
        let light = LightSource {
            intensity: Color::WHITE,
            pos: V4::new_point(0.0, 0.0, -10.0)
        };
        let pos = V4::new_point(0.0, 0.0, 0.0);

        let val = lighting(&MATERIAL, &light, &pos, &eyev, &normalv, false);
        let val = V4::from(val);

        assert!(approx_eq!(V4, val, V4::new_vector(1.0, 1.0, 1.0), epsilon = 0.0001));
    }

    #[test]
    fn opposite_surface() {
        let eyev = V4::new_vector(0.0, 0.0, -1.0);
        let normalv = V4::new_vector(0.0, 0.0, -1.0);
        let light = LightSource {
            intensity: Color::WHITE,
            pos: V4::new_point(0.0, 10.0, -10.0)
        };
        let pos = V4::new_point(0.0, 0.0, 0.0);

        let val = V4::from(lighting(&MATERIAL, &light, &pos, &eyev, &normalv, false));
        let exp = V4::new_vector(0.7364, 0.7364, 0.7364);

        assert!(approx_eq!(V4, val, exp, epsilon = 0.0001));
    }

    #[test]
    fn reflect_light() {
        let sq2half = 0.5 * std::f32::consts::SQRT_2;
        let eyev = V4::new_vector(0.0, -sq2half, -sq2half);
        let normalv = V4::new_vector(0.0, 0.0, -1.0);
        let light = LightSource {
            intensity: Color::WHITE,
            pos: V4::new_point(0.0, 10.0, -10.0)
        };
        let pos = V4::new_point(0.0, 0.0, 0.0);

        let val = V4::from(lighting(&MATERIAL, &light, &pos, &eyev, &normalv, false));
        let exp = V4::new_vector(1.6364, 1.6364, 1.6364);

        assert!(approx_eq!(V4, val, exp, epsilon = 0.0001));
    }

    #[test]
    fn in_shadow() {
        let eyev = V4::new_vector(0.0, 0.0, -1.0);
        let normalv = V4::new_vector(0.0, 0.0, -1.0);
        let light = LightSource {
            intensity: Color::WHITE,
            pos: V4::new_point(0.0, 0.0, 10.0)
        };
        let pos = V4::new_point(0.0, 0.0, 0.0);

        let val = lighting(&MATERIAL, &light, &pos, &eyev, &normalv, true);

        assert_eq!(val, V4::new_vector(0.1, 0.1, 0.1));
    }
}
