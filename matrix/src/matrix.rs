use apolaki_tuple::Tuple;
use core::array::from_fn;
use std::ops::{Index, IndexMut, Mul, Range};
use thiserror::Error;

// MxN = RxC
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BaseMatrix<const M: usize, const N: usize> {
    matrix: [[f64; N]; M],
}

// Do I even need this
#[derive(Error, Debug)]
pub enum MatrixError {
    #[error("invalid operation: {reason:?}")]
    InvalidOperationError { reason: String },

    #[error("out of bounds: idx = {idx} on range {limit:?}")]
    IndexOutOfBoundsError { limit: Range<usize>, idx: usize },
}

impl<const M: usize, const N: usize> BaseMatrix<M, N> {
    pub fn new<T: Into<f64>>(elem: T) -> Self {
        Self {
            matrix: [[elem.into(); N]; M],
        }
    }

    pub fn identity() -> Self {
        let mut init = [[0.0; N]; M];
        (0..M).into_iter().for_each(|r| {
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
        (0..M).into_iter().for_each(|r| {
            (0..N).into_iter().for_each(|c| {
                m[r][c] = self[c][r];
            })
        });
        m
    }
}

pub trait Submatrix<const M: usize, const N: usize> {
    fn submatrix(&self, r: usize, c: usize) -> Result<BaseMatrix<M, N>, MatrixError>;
}

impl Submatrix<3, 3> for Matrix4x4 {
    fn submatrix(&self, r: usize, c: usize) -> Result<BaseMatrix<3, 3>, MatrixError> {
        todo!()
    }
}

impl Submatrix<2, 2> for Matrix3x3 {
    fn submatrix(&self, r: usize, c: usize) -> Result<BaseMatrix<2, 2>, MatrixError> {
        let mut b = [[1.0; 2]; 2];
        Ok(Matrix2x2::from(b))
    }
}

impl Submatrix<1, 1> for Matrix2x2 {
    fn submatrix(&self, r: usize, c: usize) -> Result<BaseMatrix<1, 1>, MatrixError> {
        Err(MatrixError::InvalidOperationError {
            reason: "cannot get submatrix of size 3x3".to_string(),
        })
    }
}

impl<const M: usize, const N: usize> Default for BaseMatrix<M, N> {
    fn default() -> Self {
        BaseMatrix::new(0)
    }
}

type SquareMatrix<const N: usize> = BaseMatrix<N, N>;
pub type Matrix2x2 = SquareMatrix<2>;
pub type Matrix3x3 = SquareMatrix<3>;
pub type Matrix4x4 = SquareMatrix<4>;

impl<const M: usize, const N: usize> From<[[f64; N]; M]> for BaseMatrix<M, N> {
    fn from(matrix: [[f64; N]; M]) -> Self {
        Self { matrix }
    }
}

impl<const M: usize, const N: usize> Index<usize> for BaseMatrix<M, N> {
    type Output = [f64; N];

    fn index(&self, index: usize) -> &Self::Output {
        self.matrix.index(index)
    }
}

impl<const M: usize, const N: usize> IndexMut<usize> for BaseMatrix<M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.matrix.index_mut(index)
    }
}

impl<const M: usize, const N: usize> Mul<BaseMatrix<M, N>> for BaseMatrix<M, N> {
    type Output = Self;

    fn mul(self, rhs: BaseMatrix<M, N>) -> Self::Output {
        let mut m = Self::new(0.0);
        for r in 0..M {
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

impl Mul<Tuple> for BaseMatrix<4, 4> {
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
