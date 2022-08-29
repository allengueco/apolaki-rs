mod matrix {
    // MxN = RxC
    #[derive(Debug)]
    pub struct BaseMatrix<const M: usize, const N: usize> {
        matrix: [[f64; N]; M],
    }
    impl<const M: usize, const N: usize> BaseMatrix<M, N> {
        pub fn new<T: Into<f64>>(elem: T) -> Self {
            Self {
                matrix: [[elem.into(); N]; M],
            }
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
}

use matrix::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructing_and_inspecting_4x4() {
        macro_rules! matrix {
            (($lit: literal) +) => {
                [$(f64::from($lit)),+]
            }
            // (($expr: expr);+) => {
            //     [$(matrix!($expr))+]
            // };
        };
        let m = matrix! {
            3 3 3;
        };

        println!("{:?}", m);
    }
}
