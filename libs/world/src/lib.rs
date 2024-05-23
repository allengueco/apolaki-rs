use apolaki_color::Color;
use apolaki_light::Light;
use apolaki_objects::{Intersect, Sphere};
use apolaki_tuple::point;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub struct World<'a, O: Intersect> {
    objs: &'a [O],
    light: Option<Light>,
}
impl<'a, O: Intersect> World<'a, O> {
    pub fn new() -> Self {
        Self {
            objs: &[],
            light: None,
        }
    }
    pub fn size(&self) -> usize {
        self.objs.len()
    }

    pub fn light(&self) -> Option<&Light> {
        self.light.as_ref()
    }
}
impl<'a, O: Intersect> Default for World<'a, O> {
    fn default() -> Self {
        let mut s1 = Sphere::default();

        let mut s2 = Sphere::default();
        let light = Light::new(point(-10, 10, -10), Color::new(1, 1, 1));
        Self {
            objs: &[s1, s2],
            light: Some(light),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert_eq!(0, w.size());
        assert_eq!(None, w.light());
    }

    #[test]
    fn default_world() {
        let light = Light(point(-10, 10, -10), Color::new(1, 1, 1));

        let m = Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            spcular: 0.2,
            ..Default::default()
        };

        let w = World::default();
    }
    #[test]
    fn intersect_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));

        let xs = w.hit(ray);
    }
}
