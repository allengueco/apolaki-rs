use apolaki_matrix::BaseMatrix;
use crate::intersect::{Intersect, Intersection, Intersections};
use apolaki_ray::Ray;
use apolaki_tuple::point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub radius: f64,
    pub transform: BaseMatrix<4>
}

impl Sphere {
    pub fn new(radius: f64) -> Self {
        Self { radius, transform: BaseMatrix::identity() }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self { radius: 1.0, transform: BaseMatrix::identity() }
    }
}

impl Intersect for Sphere {
    type O = Self;
    fn intersect(&self, ray: Ray) -> Option<Intersections<Self::O>> {
        let sphere_to_ray = ray.origin - point(0, 0, 0);
        let a = ray.dir.dot(ray.dir);
        let b = 2. * ray.dir.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            None
        } else {
            Some(Intersections::from(Vec::from([
                Intersection::new((-b - discriminant.sqrt()) / (2. * a), *self),
                Intersection::new((-b + discriminant.sqrt()) / (2. * a), *self),
            ])))
        }
    }
}
