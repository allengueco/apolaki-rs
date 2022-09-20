mod tuple;

pub use tuple::*;
#[cfg(test)]
mod tuple_tests {
    use super::*;

    #[test]
    fn a_tuple_with_w_equals_1_is_a_point() {
        let a: Tuple = (4.3, -4.2, 3.1, 1.0).into();

        assert_eq!(4.3, a.x());
        assert_eq!(-4.2, a.y());
        assert_eq!(3.1, a.z());
        assert_eq!(1.0, a.w());

        assert!(a.is_vec());
        assert!(!a.is_point());
    }

    #[test]
    fn a_tuple_with_w_equals_0_is_a_vector() {
        let a: Tuple = (4.3, -4.2, 3.1, 0.0).into();

        assert_eq!(4.3, a.x());
        assert_eq!(-4.2, a.y());
        assert_eq!(3.1, a.z());
        assert_eq!(0.0, a.w());

        assert!(a.is_point());
        assert!(!a.is_vec());
    }

    #[test]
    fn point_constructor_creates_w_1() {
        let p = point(4, -4, 3);

        let expected: Tuple = (4, -4, 3, 1).into();
        assert_eq!(expected, p);
    }

    #[test]
    fn vector_constructor_creates_w_0() {
        let p = vector(4, -4, 3);

        let expected: Tuple = (4, -4, 3, 0).into();
        assert_eq!(expected, p);
    }

    #[test]
    fn adding_two_tuples() {
        let a1: Tuple = (3, -2, 5, 1).into();
        let a2: Tuple = (-2, 3, 1, 0).into();

        let expected: Tuple = (1, 1, 6, 1).into();
        let actual = a1 + a2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3, 2, 1);
        let p2 = point(5, 6, 7);

        let expected = vector(-2, -4, -6);
        let actual = p1 - p2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point(3, 2, 1);
        let v = vector(5, 6, 7);

        let expected = point(-2, -4, -6);
        let actual = p - v;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3, 2, 1);
        let v2 = vector(5, 6, 7);

        let expected = vector(-2, -4, -6);
        let actual = v1 - v2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = vector(0, 0, 0);
        let v = vector(1, -2, 3);

        let expected = vector(-1, 2, -3);
        let actual = zero - v;

        assert_eq!(expected, actual);
    }

    #[test]
    fn negative_tuples() {
        let a: Tuple = (1, -2, 3, -4).into();

        assert_eq!(Tuple::from((-1, 2, -3, 4)), -a);
    }

    #[test]
    fn multiply_a_tuple_by_a_scalar() {
        let a: Tuple = (1, -2, 3, -4).into();

        let expected: Tuple = (3.5, -7, 10.5, -14).into();

        assert_eq!(expected, a * 3.5);
    }

    #[test]
    fn multiply_a_tuple_by_a_fraction() {
        let a: Tuple = (1, -2, 3, -4).into();

        let expected: Tuple = (0.5, -1, 1.5, -2).into();

        assert_eq!(expected, a * 0.5);
    }

    #[test]
    fn divide_a_tuple_by_a_scalar() {
        let a: Tuple = (1, -2, 3, -4).into();

        let expected: Tuple = (0.5, -1, 1.5, -2).into();

        assert_eq!(expected, a / 2);
    }

    #[test]
    fn compute_length_vector_1_0_0() {
        let v = vector(1, 0, 0);

        assert_eq!(1., v.length());
    }

    #[test]
    fn compute_length_vector_0_1_0() {
        let v = vector(0, 1, 0);

        assert_eq!(1., v.length());
    }

    #[test]
    fn compute_length_vector_0_0_1() {
        let v = vector(0, 0, 1);

        assert_eq!(1., v.length());
    }

    #[test]
    fn compute_length_vector_1_2_3() {
        let v = vector(1, 2, 3);

        assert_eq!(14_f64.sqrt(), v.length());
    }

    #[test]
    fn compute_length_vector_1_2_3_negated() {
        let v = vector(-1, -2, -3);

        assert_eq!(14_f64.sqrt(), v.length());
    }

    #[test]
    fn normalize_vector_4_0_0_gives_vector_1_0_0() {
        let v = vector(4, 0, 0);

        assert_eq!(vector(1, 0, 0), v.normalize());
    }

    #[test]
    fn normalize_vector_1_2_3() {
        let v = vector(1, 2, 3);

        assert_eq!(vector(0.26727, 0.53452, 0.80178), v.normalize());
    }

    #[test]
    fn length_of_normalize_vector() {
        let v = vector(1, 2, 3);

        assert_eq!(1., v.normalize().length());
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);

        assert_eq!(20., a.dot(b));
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);

        assert_eq!(vector(-1, 2, -1), a.cross(b));
        assert_eq!(vector(1, -2, 1), b.cross(a));
    }
}
