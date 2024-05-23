use apolaki_color::Color;
use apolaki_light::Light;
use apolaki_tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn lighting(
        &self,
        light: &Light,
        point: Tuple,
        eye_vector: Tuple,
        normal_vector: Tuple,
    ) -> Color {
        let effective_color = self.color * light.intensity;

        let light_vector = (light.position - point).normalize();

        let ambient = effective_color * self.ambient;

        let light_dot_normal = light_vector.dot(normal_vector);

        if light_dot_normal < 0. {
            // light is on the other side of the surface
            ambient + (Color::BLACK * 2)
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            let specular = {
                let reflect_vector = -light_vector.reflect(normal_vector);
                let reflect_dot_eye = reflect_vector.dot(eye_vector);

                if reflect_dot_eye <= 0. {
                    Color::BLACK
                } else {
                    let factor = reflect_dot_eye.powf(self.shininess);

                    light.intensity * self.specular * factor
                }
            };
            ambient + diffuse + specular
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: (1, 1, 1).into(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod material_tests {
    use std::f64::consts::SQRT_2;

    use apolaki_color::Color;
    use apolaki_light::Light;
    use apolaki_tuple::*;

    use super::*;

    fn test_material() -> Material {
        Material::default()
    }

    fn test_position() -> Tuple {
        point(0, 0, 0)
    }

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(Color::from((1, 1, 1)), m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200.0, m.shininess);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = Light::new(point(0, 0, -10), (1, 1, 1).into());

        let result = test_material().lighting(&light, test_position(), eyev, normalv);

        assert_eq!(Color::new(1.9, 1.9, 1.9), result)
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
        let eyev = vector(0, SQRT_2 / 2., -SQRT_2 / 2.);
        let normalv = vector(0, 0, -1);
        let light = Light::new(point(0, 0, -10), (1, 1, 1).into());

        let result = test_material().lighting(&light, test_position(), eyev, normalv);

        assert_eq!(Color::new(1., 1., 1.), result)
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_light_offset_45_degrees() {
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = Light::new(point(0, 10, -10), (1, 1, 1).into());

        let result = test_material().lighting(&light, test_position(), eyev, normalv);

        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result)
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let eyev = vector(0, -SQRT_2 / 2., -SQRT_2 / 2.);
        let normalv = vector(0, 0, -1);
        let light = Light::new(point(0, 10, -10), (1, 1, 1).into());

        let result = test_material().lighting(&light, test_position(), eyev, normalv);

        assert_eq!(Color::new(1.6364, 1.6364, 1.6364), result)
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = Light::new(point(0, 0, 10), (1, 1, 1).into());

        let result = test_material().lighting(&light, test_position(), eyev, normalv);

        assert_eq!(Color::new(0.1, 0.1, 0.1), result)
    }
}
