use apolaki_ray::Ray;
use std::ops::Index;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersections<O: Intersect> {
    intersections: Vec<Intersection<O>>,
}

impl<O: Intersect> Intersections<O> {
    pub fn from(intersections: Vec<Intersection<O>>) -> Self {
        Self { intersections }
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }

    pub fn hit(&self) -> Option<&Intersection<O>> {
        self.intersections
            .iter()
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

#[cfg(test)]
mod intersections_test {
    use crate::Sphere;
    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
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
