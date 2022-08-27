use apolaki_tuple::Tuple;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Color(Tuple);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Color {} {} {}", self.r(), self.g(), self.b()))
    }
}

impl Default for Color {
    // default to white
    fn default() -> Self {
        (0, 0, 0).into()
    }
}

impl Color {
    #[inline]
    pub fn r(self) -> f64 {
        self.0.x()
    }

    #[inline]
    pub fn g(self) -> f64 {
        self.0.y()
    }

    #[inline]
    pub fn b(self) -> f64 {
        self.0.z()
    }

    #[inline]
    pub fn to_ppm_color(&self) -> String {
        // Don't know if lerp-ing refers to this
        fn lerp(n: f64) -> u8 {
            (n.clamp(0., 0.999) * 256.) as u8
        }
        let [r, g, b] = [lerp(self.r()), lerp(self.g()), lerp(self.b())];

        return format!("{} {} {}", r, g, b);
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<R, G, B> From<(R, G, B)> for Color
where
    R: Into<f64>,
    G: Into<f64>,
    B: Into<f64>,
{
    fn from(t: (R, G, B)) -> Self {
        Self(Tuple::vector(t.0.into(), t.1.into(), t.2.into()))
    }
}

impl Add<Self> for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Self> for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        let inner = (
            self.0.x() * rhs.0.x(),
            self.0.y() * rhs.0.y(),
            self.0.z() * rhs.0.z(),
            self.0.w() * rhs.0.w(),
        )
            .into();
        Self(inner)
    }
}

impl<N> Mul<N> for Color
where
    N: Into<f64> + Copy + Clone,
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        let inner = (
            self.0.x() * rhs.into(),
            self.0.y() * rhs.into(),
            self.0.z() * rhs.into(),
            self.0.w() * rhs.into(),
        )
            .into();
        Self(inner)
    }
}
