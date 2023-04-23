mod objects {
    use std::ops::Index;

    use apolaki_ray::Ray;

    pub type Intersections<O> = Vec<Intersection<O>>;

    pub struct Intersection<O: Intersect> {
        pub t: f64,
        pub obj: O,
    }

    impl<O: Intersect> Intersection<O> {
        pub fn new<T: Into<f64>>(t: T, obj: O) -> Self {
            Self { t: t.into(), obj }
        }
    }

    impl<O> Index<usize> for Intersection<O> where O: Intersect {
        type Output = Intersection<O>;

        fn index(&self, index: usize) -> &Self::Output {
            self
        }
    }

    pub trait Intersect: Sized {
        fn intersect(&self, ray: Ray) -> Option<Intersections<Self>>;
    }
}

mod sphere {
    use crate::objects::{Intersect, Intersection, Intersections};
    use apolaki_ray::Ray;
    use apolaki_tuple::point;

    #[derive(Clone, Copy, Debug)]
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
        fn intersect(&self, ray: Ray) -> Option<Intersections<Self>> {
            let sphere_to_ray = ray.origin - point(0, 0, 0);
            let a = ray.dir.dot(ray.dir);
            let b = 2. * ray.dir.dot(sphere_to_ray);
            let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

            let discriminant = b.powi(2) - 4. * a * c;

            if discriminant < 0. {
                None
            } else {
                Some(Vec::from([
                    Intersection::new((-b - discriminant.sqrt()) / (2. * a), self.clone()),
                    Intersection::new((-b + discriminant.sqrt()) / (2. * a), self.clone()),
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
    }

    #[cfg(test)]
    mod intersections_test {
        use super::*;

        #[test]
        fn an_interesection_encapsulates_t_and_object() {
            let s = Sphere::default();

            let i = Intersection::new(3.5, s);

            assert_eq!(i.t, 3.5);
            assert_eq!(i.obj, s);
        }

        #[test]
        fn aggregating_intersectinos() {
            let s = Sphere::default();
            let i1 = Intersection::new(1, s);
            let i2 = Intersection::new(2, s);

            let xs = vec![i1, i2];

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, 1.0);
            assert_eq!(xs[1].t, 2.0);
        }
    }
}
