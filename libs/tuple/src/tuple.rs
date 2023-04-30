use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Tuple(pub f64, pub f64, pub f64, pub f64);

impl Tuple {
    #[inline]
    pub fn x(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.1
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.2
    }

    #[inline]
    pub fn w(&self) -> f64 {
        self.3
    }

    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.0 = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.1 = y;
    }

    #[inline]
    pub fn set_z(&mut self, z: f64) {
        self.2 = z;
    }

    #[inline]
    pub fn set_w(&mut self, w: f64) {
        self.3 = w;
    }

    #[inline]
    pub fn is_point(&self) -> bool {
        self.w() == 1.0
    }

    #[inline]
    pub fn is_vec(&self) -> bool {
        self.w() == 0.0
    }

    #[inline]
    pub fn length(&self) -> f64 {
        [self.0, self.1, self.2, self.3]
            .iter()
            .map(|n| n.powi(2))
            .sum::<f64>()
            .sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        Self(
            self.0 / self.length(),
            self.1 / self.length(),
            self.2 / self.length(),
            self.3 / self.length(),
        )
    }

    #[inline]
    pub fn dot(self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        vector(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    #[inline]
    pub fn reflect(&self, normal: Tuple) -> Self {
        *self - normal * 2 * self.dot(normal)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 0.00001;
        fn equal(t: (f64, f64)) -> bool {
            (t.0 - t.1).abs() < EPSILON
        }

        let x = [self.0, self.1, self.2, self.3].into_iter();
        let y = [other.0, other.1, other.2, other.3].into_iter();

        x.zip(y).all(equal)
    }
}

impl Add<Tuple> for Tuple {
    type Output = Self;

    fn add(self, rhs: Tuple) -> Self::Output {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Sub<Tuple> for Tuple {
    type Output = Self;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

impl<N> Mul<N> for Tuple
where
    N: Into<f64> + Copy + Clone,
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self(
            self.0 * rhs.into(),
            self.1 * rhs.into(),
            self.2 * rhs.into(),
            self.3 * rhs.into(),
        )
    }
}

impl Mul<Tuple> for Tuple {
    type Output = Self;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Self(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl<N> Div<N> for Tuple
where
    N: Into<f64> + Copy + Clone,
{
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Self(
            self.0 / rhs.into(),
            self.1 / rhs.into(),
            self.2 / rhs.into(),
            self.3 / rhs.into(),
        )
    }
}

impl<X, Y, Z, W> From<(X, Y, Z, W)> for Tuple
where
    X: Into<f64>,
    Y: Into<f64>,
    Z: Into<f64>,
    W: Into<f64>,
{
    fn from(t: (X, Y, Z, W)) -> Self {
        Self(t.0.into(), t.1.into(), t.2.into(), t.3.into())
    }
}

#[inline]
pub fn point<X, Y, Z>(x: X, y: Y, z: Z) -> Tuple
where
    X: Into<f64>,
    Y: Into<f64>,
    Z: Into<f64>,
{
    Tuple(x.into(), y.into(), z.into(), 1.0)
}

#[inline]
pub fn vector<X, Y, Z>(x: X, y: Y, z: Z) -> Tuple
where
    X: Into<f64>,
    Y: Into<f64>,
    Z: Into<f64>,
{
    Tuple(x.into(), y.into(), z.into(), 0.0)
}

#[cfg(test)]
mod tuple_tests {
    use super::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn a_tuple_with_w_equals_1_is_a_point() {
        let a: Tuple = (4.3, -4.2, 3.1, 1.0).into();

        assert_eq!(4.3, a.x());
        assert_eq!(-4.2, a.y());
        assert_eq!(3.1, a.z());
        assert_eq!(1.0, a.w());

        assert!(!a.is_vec());
        assert!(a.is_point());
    }

    #[test]
    fn a_tuple_with_w_equals_0_is_a_vector() {
        let a: Tuple = (4.3, -4.2, 3.1, 0.0).into();

        assert_eq!(4.3, a.x());
        assert_eq!(-4.2, a.y());
        assert_eq!(3.1, a.z());
        assert_eq!(0.0, a.w());

        assert!(!a.is_point());
        assert!(a.is_vec());
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

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = vector(1, -1, 0);
        let n = vector(0, 1, 0);

        let r = v.reflect(n);

        assert_eq!(vector(1, 1, 0), r);
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = vector(0, -1, 0);
        let n = vector(SQRT_2 / 2., SQRT_2 / 2., 0);

        let r = v.reflect(n);

        assert_eq!(vector(1, 0, 0), r);
    }
}
