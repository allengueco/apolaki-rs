mod objects {
    use std::ops::Index;
    use apolaki_ray::Ray;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Intersections<O: Intersect> {
        intersections: Vec<Intersection<O>>
    }

    impl<O: Intersect> Intersections<O> {
        pub fn from(intersections: Vec<Intersection<O>>) -> Self {
            Self {
                intersections
            }
        }

        pub fn len(&self) -> usize {
            self.intersections.len()
        }

        pub fn is_empty(&self) -> bool {
            self.intersections.is_empty()
        }

        pub fn hit(&self) -> Option<&Intersection<O>> {
            self.intersections.iter()
                .filter(|i| i.t > 0.)
                .min_by(|i1, i2| i1.t.total_cmp(&i2.t))
        }
    }

    impl<O: Intersect> Index<usize> for Intersections<O> {
        type Output = Intersection<O>;

        fn index(&self, index: usize) -> &Self::Output {
            self.intersections.index(index)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Intersection<O: Intersect> {
        pub t: f64,
        pub obj: O,
    }

    impl<O: Intersect> Intersection<O> {
        pub fn new<T: Into<f64>>(t: T, obj: O) -> Self {
            Self { t: t.into(), obj }
        }
    }

    pub trait Intersect {
        type O: Intersect;
        fn intersect(&self, ray: Ray) -> Option<Intersections<Self::O>>;
    }
}

mod sphere {
    use crate::objects::{Intersect, Intersection, Intersections};
    use apolaki_ray::Ray;
    use apolaki_tuple::point;

    #[derive(Clone, Copy, Debug, PartialEq)]
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
                    Intersection::new((-b - discriminant.sqrt()) / (2. * a), self.clone()),
                    Intersection::new((-b + discriminant.sqrt()) / (2. * a), self.clone()),
                ])))
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

        #[test]
        fn intersect_sets_the_object_on_the_intersection() {
            let ray = Ray::new(point(0, 0, -5), vector(0, 0, 1));
            let s = Sphere::default();

            let xs = s.intersect(ray).expect("Must intersect");

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].obj, s);
            assert_eq!(xs[1].obj, s);
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
        fn aggregating_intersections() {
            let s = Sphere::default();
            let i1 = Intersection::new(1, s);
            let i2 = Intersection::new(2, s);

            let xs = Intersections::from(vec![i1, i2]);

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, 1.0);
            assert_eq!(xs[1].t, 2.0);
        }

        #[test]
        fn the_hit_when_all_intersections_have_positive_t() {
            let s = Sphere::default();
            let i1 = Intersection::new(1, s);
            let i2 = Intersection::new(2, s);
            let xs = Intersections::from(vec![i2, i1]);

            let i = xs.hit();

            assert_eq!(i, Some(&i1));
        }

        #[test]
        fn the_hit_when_some_intersections_have_negative_t() {
            let s = Sphere::default();
            let i1 = Intersection::new(-1, s);
            let i2 = Intersection::new(1, s);
            let xs = Intersections::from(vec![i2, i1]);

            let i = xs.hit();

            assert_eq!(i, Some(&i2));
        }

        #[test]
        fn the_hit_when_all_intersections_have_negative_t() {
            let s = Sphere::default();
            let i1 = Intersection::new(-2, s);
            let i2 = Intersection::new(-1, s);
            let xs = Intersections::from(vec![i2, i1]);

            let i = xs.hit();

            assert_eq!(i, None);
        }

        #[test]
        fn the_hit_is_always_the_lowest_nonnegative_intersection() {
            let s = Sphere::default();
            let i1 = Intersection::new(5, s);
            let i2 = Intersection::new(7, s);
            let i3 = Intersection::new(-3, s);
            let i4 = Intersection::new(2, s);
            let xs = Intersections::from(vec![i1, i2, i3, i4]);

            let i = xs.hit();

            assert_eq!(i, Some(&i4));
        }
    }
}
