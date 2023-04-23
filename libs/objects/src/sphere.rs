use apolaki_matrix::{BaseMatrix, Invert};
use apolaki_ray::Ray;
use apolaki_transform::Transform;
use apolaki_tuple::point;

use crate::intersect::{Intersect, Intersection, Intersections};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub radius: f64,
    pub transform: BaseMatrix<4>,
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
        let r = ray.transform(self.transform.invert());
        let sphere_to_ray = r.origin - point(0, 0, 0);
        let a = r.dir.dot(r.dir);
        let b = 2. * r.dir.dot(sphere_to_ray);
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

#[cfg(test)]
mod sphere_tests {
    use apolaki_ray::Ray;
    use apolaki_tuple::{point, vector};

    use super::*;

    #[test]
    fn ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r).expect("must have intersections");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_a_sphere_at_tangent() {
        let r = Ray::new(point(0, 1, -5), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r).expect("must have intersections");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(point(0, 2, -5), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r);
        assert_eq!(xs, None);
    }

    #[test]
    fn ray_originates_inside() {
        let r = Ray::new(point(0, 0, 0), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r).expect("must have intersections");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(point(0, 0, 5), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r).expect("must have intersections");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(ray).expect("Must intersect");

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].obj, s);
        assert_eq!(xs[1].obj, s);
    }

    #[test]
    fn a_spheres_default_transform() {
        let s = Sphere::default();

        assert_eq!(BaseMatrix::identity(), s.transform);
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::default();

        let m = BaseMatrix::identity().translate(2, 3, 4);
        s.transform = m;

        assert_eq!(BaseMatrix::identity().translate(2, 3, 4), s.transform);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::default();

        s.transform = BaseMatrix::identity().scale(2, 2, 2);
        let xs = s.intersect(r).expect("Must intersect");

        assert_eq!(2, xs.len());

        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::default();

        s.transform = BaseMatrix::identity().translate(5, 0, 0);
        let xs = s.intersect(r);

        assert_eq!(None, xs);
    }
}

