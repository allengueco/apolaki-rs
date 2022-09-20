mod objects {
    use apolaki_ray::Ray;

    pub type Intersections = Vec<f64>;

    pub trait Intersect {
        fn intersect(&self, ray: Ray) -> Option<Intersections>;
    }
}

mod sphere {
    use crate::objects::{Intersect, Intersections};
    use apolaki_ray::Ray;
    use apolaki_tuple::point;

    pub struct Sphere {
        pub radius: f64,
    }

    impl Sphere {
        pub fn new(radius: f64) -> Self {
            Self { radius }
        }
    }

    impl Default for Sphere {
        fn default() -> Self {
            Self { radius: 1.0 }
        }
    }

    impl Intersect for Sphere {
        fn intersect(&self, ray: Ray) -> Option<Intersections> {
            let sphere_to_ray = ray.origin - point(0, 0, 0);
            let a = ray.dir.dot(ray.dir);
            let b = 2. * ray.dir.dot(sphere_to_ray);
            let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

            let discriminant = b.powi(2) - 4. * a * c;

            if discriminant < 0. {
                None
            } else {
                Some(Vec::from([
                    (-b - discriminant.sqrt()) / (2. * a),
                    (-b + discriminant.sqrt()) / (2. * a),
                ]))
            }
        }
    }
}
pub use objects::*;
pub use sphere::*;

#[cfg(test)]
mod objects_tests {
    use super::*;

    #[cfg(test)]
    mod sphere {
        use super::*;
        use apolaki_ray::Ray;
        use apolaki_tuple::{point, vector};

        #[test]
        fn ray_intersects_a_sphere_at_two_points() {
            let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
            let s = Sphere::default();
            let xs = s.intersect(r).expect("must have intersections");
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], 4.0);
            assert_eq!(xs[1], 6.0);
        }

        #[test]
        fn ray_intersects_a_sphere_at_tangent() {
            let r = Ray::new(point(0, 1, -5), vector(0, 0, 1));
            let s = Sphere::default();
            let xs = s.intersect(r).expect("must have intersections");
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], 5.0);
            assert_eq!(xs[1], 5.0);
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
            assert_eq!(xs[0], -1.0);
            assert_eq!(xs[1], 1.0);
        }

        #[test]
        fn sphere_is_behind_ray() {
            let r = Ray::new(point(0, 0, 5), vector(0, 0, 1));
            let s = Sphere::default();
            let xs = s.intersect(r).expect("must have intersections");
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], -6.0);
            assert_eq!(xs[1], -4.0);
        }
    }
}
