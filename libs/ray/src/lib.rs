mod ray {
    use apolaki_tuple::Tuple;

    #[derive(Clone, Copy, Debug)]
    pub struct Ray {
        pub origin: Tuple,
        pub dir: Tuple,
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
        let direction = vector(4, 5, 6);
    }
}
