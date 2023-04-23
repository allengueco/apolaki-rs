pub use ray::*;

mod ray {
    use apolaki_matrix::BaseMatrix;
    use apolaki_transform::*;
    use apolaki_tuple::{Tuple};

    #[derive(Clone, Copy, Debug)]
    pub struct Ray {
        pub origin: Tuple,
        pub dir: Tuple,
    }

    impl Ray {
        pub fn new(origin: Tuple, dir: Tuple) -> Self {
            Self { origin, dir }
        }
        pub fn position<T: Into<f64>>(&self, t: T) -> Tuple {
            self.origin + self.dir * t.into()
        }
    }

    impl Transform for Ray {
        fn transform(&self, m: BaseMatrix<4>) -> Self {
            Self {
                origin: m * self.origin,
                dir: m * self.dir,
            }
        }
    }
}

#[cfg(test)]
mod ray_tests {
    use apolaki_matrix::BaseMatrix;
    use apolaki_transform::*;
    use apolaki_tuple::{point, vector};

    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1, 2, 3);
        let dir = vector(4, 5, 6);

        let r = Ray::new(origin, dir);

        assert_eq!(origin, r.origin);
        assert_eq!(dir, r.dir);
    }

    #[test]
    fn computing_point_from_distance() {
        let r = Ray::new(point(2, 3, 4), vector(1, 0, 0));

        assert_eq!(point(2, 3, 4), r.position(0));
        assert_eq!(point(3, 3, 4), r.position(1));
        assert_eq!(point(1, 3, 4), r.position(-1));
        assert_eq!(point(4.5, 3, 4), r.position(2.5));
    }

    #[test]
    fn transforming_a_ray() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));

        let m = BaseMatrix::identity().translate(3, 4, 5);
        dbg!(&m);

        let r2 = r.transform(m);
        assert_eq!(point(4, 6, 8), r2.origin);
        assert_eq!(vector(0, 1, 0), r2.dir);
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));

        let m = BaseMatrix::identity().scale(2, 3, 4);

        let r2 = r.transform(m);
        assert_eq!(point(2, 6, 12), r2.origin);
        assert_eq!(vector(0, 3, 0), r2.dir);
    }
}
