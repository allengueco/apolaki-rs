use apolaki_tuple::Tuple;
use core::array::from_fn;
use std::ops::{Index, IndexMut, Mul};

// MxN = RxC
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BaseMatrix<const N: usize> {
    matrix: [[f64; N]; N],
}

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

    // Relying on #![feature(generic_const_exprs)]
    // Since I can't generically declare a method which returns a BaseMatrix<M-1, N-1> (const
    // generic expr not implemented) yet
    // https://doc.rust-lang.org/unstable-book/language-features/generic-const-exprs.html
    pub fn submatrix(&self, r: usize, c: usize) -> BaseMatrix<{ N - 1 }>
    where
        [(); N - 1]:,
    {
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
        let mut b = [[0.0; N - 1]; N - 1];
        let vec = remove(&self.matrix, r, c);
        for (r1, r2) in b.iter_mut().zip(vec.iter()) {
            r1.copy_from_slice(r2.as_slice());
        }
        BaseMatrix::<{ N - 1 }>::from(b)
    }

    pub fn determinant(&self) -> f64
        where [(); N - 1]: Sized,
    {
        if N == 2 {
            self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0]
        } else {
            self[0]
                .iter()
                .enumerate()
                .map(|(i, e)| e * self.cofactor(0, i))
                .sum()
        }
    }

    pub fn minor(&self, r: usize, c: usize) -> f64
    {
        let submatrix: BaseMatrix<{N - 1}> = self.submatrix(r, c);
        submatrix.determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f64
    where
        [(); N - 1]:,
    {
        if (r + c) % 2 == 0 {
            self.minor(r, c)
        } else {
            -self.minor(r, c)
        }
    }
}

impl<const N: usize> Default for BaseMatrix<N> {
    fn default() -> Self {
        BaseMatrix::new(0)
    }
}

pub type Matrix2x2 = BaseMatrix<2>;
pub type Matrix3x3 = BaseMatrix<3>;
pub type Matrix4x4 = BaseMatrix<4>;

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

#[macro_export]
macro_rules! matrix {
    ($( $( $x:literal )*; )*) => {
        BaseMatrix::from(
        [$(
            [ $( f64::from($x) ),* ]),*
        ])
    };
}
