mod ray {
    use apolaki_tuple::Tuple;

    #[derive(Clone, Copy, Debug)]
    pub struct Ray {
        pub origin: Tuple,
        pub dir: Tuple,
    }

    impl Ray {
        pub fn position<T: Into<f64>>(&self, t: T) -> Tuple {
            self.origin + self.dir * t.into()
        }
    }
}

pub use ray::*;

#[cfg(test)]
mod tests {
    use apolaki_tuple::{point, vector};
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1, 2, 3);
        let dir = vector(4, 5, 6);

        let r = Ray { origin, dir };

        assert_eq!(origin, r.origin);
        assert_eq!(dir, r.dir);
    }

    #[test]
    fn computing_point_from_distance() {
        let r = Ray {
            origin: point(2, 3, 4),
            dir: vector(1, 0, 0),
        };

        assert_eq!(point(2, 3, 4), r.position(0));
        assert_eq!(point(3, 3, 4), r.position(1));
        assert_eq!(point(1, 3, 4), r.position(-1));
        assert_eq!(point(4.5, 3, 4), r.position(2.5));
    }
}
