mod matrix {
    use apolaki_tuple::Tuple;
    use std::ops::{Index, IndexMut, Mul};

    // MxN = RxC
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct BaseMatrix<const M: usize, const N: usize> {
        matrix: [[f64; N]; M],
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

    use core::array::from_fn;
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
}

use matrix::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructing_and_inspecting_4x4() {
        let m = matrix! {
            1 2 3 4;
            5.5 6.5 7.5 8.5;
            9 10 11 12;
            13.5 14.5 15.5 16.5;
        };

        assert_eq!(1.0, m[0][0]);
        assert_eq!(4.0, m[0][3]);
        assert_eq!(5.5, m[1][0]);
        assert_eq!(7.5, m[1][2]);
        assert_eq!(11.0, m[2][2]);
        assert_eq!(13.5, m[3][0]);
        assert_eq!(15.5, m[3][2]);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m: BaseMatrix<2, 2> = matrix! {
            -3 5;
            1 -2;
        };

        assert_eq!(-3.0, m[0][0]);
        assert_eq!(5.0, m[0][1]);
        assert_eq!(1.0, m[1][0]);
        assert_eq!(-2.0, m[1][1]);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = matrix! {
            -3 5 0;
            1 -2 -7;
            0 1 1;
        };

        assert_eq!(-3.0, m[0][0]);
        assert_eq!(-2.0, m[1][1]);
        assert_eq!(1.0, m[2][2]);
    }

    #[cfg(test)]
    mod ops {
        use super::*;
        use apolaki_tuple::Tuple;
        use std::process::id;

        #[test]
        fn matrix_eq_with_identical_matrices() {
            let a = matrix! {
                1 2 3 4;
                5 6 7 8;
                9 8 7 6;
                5 4 3 2;
            };

            let b = matrix! {
                1 2 3 4;
                5 6 7 8;
                9 8 7 6;
                5 4 3 2;
            };

            assert_eq!(a, b);
        }

        #[test]
        fn matrix_eq_with_different_matrices() {
            let a = matrix! {
                1 2 3 4;
                5 6 7 8;
                9 8 7 6;
                5 4 3 2;
            };

            let b = matrix! {
                2 3 4 5;
                6 7 8 9;
                8 7 6 5;
                4 3 2 1;
            };

            assert_ne!(a, b);
        }

        #[test]
        fn multiplying() {
            let a = matrix! {
                1 2 3 4;
                5 6 7 8;
                9 8 7 6;
                5 4 3 2;
            };

            let b = matrix! {
                -2 1 2 3;
                3 2 1 -1;
                4 3 6 5;
                1 2 7 8;
            };

            let expected = matrix! {
                20 22 50 48;
                44 54 114 108;
                40 58 110 102;
                16 26 46 42;
            };

            assert_eq!(expected, a * b);
        }

        #[test]
        fn multiplying_by_tuple() {
            let a = matrix! {
                1 2 3 4;
                2 4 4 2;
                8 6 4 1;
                0 0 0 1;
            };

            let b: Tuple = (1, 2, 3, 1).into();

            let expected: Tuple = (18, 24, 33, 1).into();

            assert_eq!(expected, a * b);
        }

        #[test]
        fn multiply_by_identity() {
            let a = matrix! {
                0 1 2 4;
                1 2 4 8;
                2 4 8 16;
                4 8 16 32;
            };

            assert_eq!(a, a * Matrix4x4::identity());
        }

        #[test]
        fn transpose() {
            let a = matrix! {
                0 9 3 0;
                9 8 0 8;
                1 8 5 3;
                0 0 5 8;
            };

            let expected = matrix! {
                0 9 1 0;
                9 8 8 0;
                3 0 5 5;
                0 8 3 8;
            };

            assert_eq!(expected, a.transpose());
        }

        #[test]
        fn transpose_the_identity() {
            let identity = Matrix4x4::identity();

            assert_eq!(identity, identity.transpose());
        }
    }
}
