use apolaki_color::Color;
use apolaki_tuple::*;

pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod light_tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::from((1, 1, 1));
        let position = point(0, 0, 0);

        let light = Light::new(position, intensity);

        assert_eq!(position, light.position);
        assert_eq!(intensity, light.intensity);
    }
}
