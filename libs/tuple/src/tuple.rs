use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Tuple(f64, f64, f64, f64);

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
    pub fn is_point(&self) -> bool {
        self.w() == 0.0
    }

    #[inline]
    pub fn is_vec(&self) -> bool {
        self.w() == 1.0
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
            self.3 * rhs.3
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
