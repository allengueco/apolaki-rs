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
