use apolaki_tuple::Tuple;
use core::array::from_fn;
use std::ops::{Index, IndexMut, Mul};

// MxN = RxC
#[derive(Debug, Copy, Clone)]
pub struct BaseMatrix<const N: usize> {
    matrix: [[f64; N]; N],
}

impl<const N: usize> PartialEq for BaseMatrix<N> {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 0.00001;

        fn array_equal(a1: &[f64], a2: &[f64]) -> bool {
            a1.iter().zip(a2.iter()).all(equal)
        }
        fn equal(t: (&f64, &f64)) -> bool {
            (t.0 - t.1).abs() < EPSILON
        }

        self.matrix
            .iter()
            .zip(other.matrix.iter())
            .all(|(a1, a2)| array_equal(a1, a2))
    }
}

// methods which require taking a submatrix
pub trait Submatrix<const N: usize> {
    fn submatrix(&self, r: usize, c: usize) -> BaseMatrix<N>;
    fn minor(&self, r: usize, c: usize) -> f64;
    fn determinant(&self) -> f64;
    fn cofactor(&self, r: usize, c: usize) -> f64;
    fn remove<const O: usize, const P: usize>(
        matrix: &[[f64; P]; O],
        r: usize,
        c: usize,
    ) -> Vec<Vec<f64>> {
        let (left, right) = matrix.split_at(r);
        let split_rows = [left, &right[1..]].concat(); // removed row at `r`
        split_rows
            .iter()
            .map(|row| {
                let (left, right) = row.split_at(c); // remove elem at 'c' from each row
                [left, &right[1..]].concat()
            })
            .collect()
    }
}

pub trait Invert<const N: usize>: Submatrix<N> {
    fn invertible(&self) -> bool {
        self.determinant() != 0.
    }

    fn invert<const M: usize>(&self) -> BaseMatrix<M> {
        if !self.invertible() {
            panic!("tried inverting non-invertible matrix")
        }
        let mut b = [[0.0; M]; M];
        (0..M).into_iter().for_each(|r| {
            (0..M).into_iter().for_each(|c| {
                let co = self.cofactor(r, c);
                let d = self.determinant();
                b[c][r] = co / d;
            })
        });

        b.into()
    }
}

impl<T> Invert<3> for T where T: Submatrix<3> {}

impl Submatrix<0> for BaseMatrix<1> {
    #[inline(always)]
    fn submatrix(&self, _r: usize, _c: usize) -> BaseMatrix<0> {
        [].into()
    }

    #[inline(always)]
    fn minor(&self, _r: usize, _c: usize) -> f64 {
        self.determinant()
    }

    #[inline(always)]
    fn determinant(&self) -> f64 {
        self[0][0]
    }

    #[inline(always)]
    fn cofactor(&self, _r: usize, _c: usize) -> f64 {
        self[0][0]
    }
}

impl Submatrix<1> for BaseMatrix<2> {
    #[inline(always)]
    fn submatrix(&self, r: usize, c: usize) -> BaseMatrix<1> {
        let mut b = [[0.0; 1]; 1];
        let vec = Self::remove(&self.matrix, r, c);
        for (r1, r2) in b.iter_mut().zip(vec.iter()) {
            r1.copy_from_slice(r2.as_slice());
        }
        BaseMatrix::from(b)
    }

    #[inline(always)]
    fn minor(&self, r: usize, c: usize) -> f64 {
        self.submatrix(r, c).determinant()
    }

    #[inline(always)]
    fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    #[inline(always)]
    fn cofactor(&self, r: usize, c: usize) -> f64 {
        if (r + c) % 2 == 0 {
            self.minor(r, c)
        } else {
            -self.minor(r, c)
        }
    }
}

// Because I can't do this generically yet, I just use macros to implement these methods
// declaratively.
// #![feature(generic_const_exprs)]
macro_rules! impl_submatrix_for_square_matrix_n {
    ($n_minus:expr, $n: expr) => {
        impl Submatrix<$n_minus> for BaseMatrix<$n> {
            #[inline(always)]
            fn submatrix(&self, r: usize, c: usize) -> BaseMatrix<$n_minus> {
                let mut b = [[0.0; $n_minus]; $n_minus];
                let vec = Self::remove(&self.matrix, r, c);
                for (r1, r2) in b.iter_mut().zip(vec.iter()) {
                    r1.copy_from_slice(r2.as_slice());
                }
                BaseMatrix::from(b)
            }

            #[inline(always)]
            fn minor(&self, r: usize, c: usize) -> f64 {
                self.submatrix(r, c).determinant()
            }

            #[inline(always)]
            fn determinant(&self) -> f64 {
                self[0]
                    .iter()
                    .enumerate()
                    .map(|(i, e)| e * self.cofactor(0, i))
                    .sum()
            }

            #[inline(always)]
            fn cofactor(&self, r: usize, c: usize) -> f64 {
                if (r + c) % 2 == 0 {
                    self.minor(r, c)
                } else {
                    -self.minor(r, c)
                }
            }
        }
    };
}
impl_submatrix_for_square_matrix_n! { 2, 3 }
impl_submatrix_for_square_matrix_n! { 3, 4 }

impl<const N: usize> BaseMatrix<N> {
    pub fn new<T: Into<f64>>(elem: T) -> Self {
        Self {
            matrix: [[elem.into(); N]; N],
        }
    }

    pub fn identity() -> Self {
        let mut init = [[0.0; N]; N];
        (0..N).into_iter().for_each(|r| {
            (0..N).into_iter().for_each(|c| {
                if r == c {
                    init[r][c] = 1.0
                }
            })
        });
        init.into()
    }

    pub fn transpose(&self) -> Self {
        let mut m = self.clone();
        (0..N).into_iter().for_each(|r| {
            (0..N).into_iter().for_each(|c| {
                m[r][c] = self[c][r];
            })
        });
        m
    }
}

impl<const N: usize> Default for BaseMatrix<N> {
    fn default() -> Self {
        BaseMatrix::new(0)
    }
}

impl<const N: usize> From<[[f64; N]; N]> for BaseMatrix<N> {
    fn from(matrix: [[f64; N]; N]) -> Self {
        Self { matrix }
    }
}

impl<const N: usize> Index<usize> for BaseMatrix<N> {
    type Output = [f64; N];

    fn index(&self, index: usize) -> &Self::Output {
        self.matrix.index(index)
    }
}

impl<const N: usize> IndexMut<usize> for BaseMatrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.matrix.index_mut(index)
    }
}

impl<const N: usize> Mul<BaseMatrix<N>> for BaseMatrix<N> {
    type Output = Self;

    fn mul(self, rhs: BaseMatrix<N>) -> Self::Output {
        let mut m = Self::new(0.0);
        for r in 0..N {
            for c in 0..N {
                m[r][c] = self[r][0] * rhs[0][c]
                    + self[r][1] * rhs[1][c]
                    + self[r][2] * rhs[2][c]
                    + self[r][3] * rhs[3][c];
            }
        }
        m
    }
}

impl Mul<Tuple> for BaseMatrix<4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        // nice
        let [x, y, z, w] = from_fn(|r| {
            let [x, y, z, w] = self[r];
            let r = Tuple::from((x, y, z, w));
            r.dot(rhs)
        });
        (x, y, z, w).into()
    }
}

impl BaseMatrix<4> {
    pub fn translate<X, Y, Z>(self, x: X, y: Y, z: Z) -> Self
        where
            X: Into<f64>,
            Y: Into<f64>,
            Z: Into<f64>,
    {
        let mut s = BaseMatrix::identity();
        s[0][3] = x.into();
        s[1][3] = y.into();
        s[2][3] = z.into();
        self * s
    }

    pub fn scale<X, Y, Z>(self, x: X, y: Y, z: Z) -> Self
        where
            X: Into<f64>,
            Y: Into<f64>,
            Z: Into<f64>, {
        let mut s = BaseMatrix::identity();
        s[0][0] = x.into();
        s[1][1] = y.into();
        s[2][2] = z.into();
        self * s
    }

    pub fn rotate_x<R>(self, radians: R) -> Self
        where
            R: Into<f64>,
    {
        let mut s = BaseMatrix::identity();
        let radians = radians.into();
        s[1][1] = radians.cos();
        s[1][2] = -radians.sin();
        s[2][1] = radians.sin();
        s[2][2] = radians.cos();
        self * s
    }
    pub fn rotate_y<R>(self, radians: R) -> Self
        where
            R: Into<f64>,
    {
        let mut s = BaseMatrix::identity();
        let radians = radians.into();
        s[0][0] = radians.cos();
        s[0][2] = radians.sin();
        s[2][0] = -radians.sin();
        s[2][2] = radians.cos();
        self * s
    }
    pub fn rotate_z<R>(self, radians: R) -> Self
        where
            R: Into<f64>,
    {
        let mut s = BaseMatrix::identity();
        let radians = radians.into();
        s[0][0] = radians.cos();
        s[0][1] = -radians.sin();
        s[1][0] = radians.sin();
        s[1][1] = radians.cos();
        self * s
    }

    pub fn shear<XY, XZ, YX, YZ, ZX, ZY>(self, xy: XY, xz: XZ, yx: YX, yz: YZ, zx: ZX, zy: ZY) -> Self
        where
            XY: Into<f64>,
            XZ: Into<f64>,
            YX: Into<f64>,
            YZ: Into<f64>,
            ZX: Into<f64>,
            ZY: Into<f64>,
    {
        let mut s = BaseMatrix::identity();
        s[0][1] = xy.into();
        s[0][2] = xz.into();
        s[1][0] = yx.into();
        s[1][2] = yz.into();
        s[2][0] = zx.into();
        s[2][1] = zy.into();
        self * s
    }
}

#[macro_export]
macro_rules! matrix {
    ($( $( $x:literal )*; )*) => {
        BaseMatrix::from(
        [$(
            [ $( f64::from($x) ),* ]),*
        ])
    };
}
