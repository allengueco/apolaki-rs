mod matrix;

pub use matrix::*;

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
        let m = matrix! {
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
        use apolaki_tuple::{Tuple};

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

            assert_eq!(a, a * BaseMatrix::identity());
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
            let identity = BaseMatrix::<4>::identity();

            assert_eq!(identity, identity.transpose());
        }

        #[test]
        fn calculating_the_determinant_of_2x2() {
            let a = matrix! {
                1 5;
                -3 2;
            };
            assert_eq!(17., a.determinant());
        }

        #[test]
        fn submatrix_of_3x3_is_2x2() {
            let a = matrix! {
                1 5 0;
                -3 2 7;
                0 6 -3;
            };

            let expected = matrix! {
                -3 2;
                0 6;
            };

            assert_eq!(expected, a.submatrix(0, 2));
        }

        #[test]
        fn submatrix_of_4x4_is_3x3() {
            let a = matrix! {
                -6 1 1 6;
                -8 5 8 6;
                -1 0 8 2;
                -7 1 -1 1;
            };

            let expected = matrix! {
                -6 1 6;
                -8 8 6;
                -7 -1 1;
            };

            assert_eq!(expected, a.submatrix(2, 1));
        }

        #[test]
        fn calculating_minor_of_3x3() {
            let a = matrix! {
                3 5 0;
                2 -1 -7;
                6 -1 5;
            };

            let b = a.submatrix(1, 0);
            assert_eq!(b.determinant(), 25.);
            assert_eq!(a.minor(1, 0), 25.);
        }

        #[test]
        fn calculating_cofactor_of_3x3() {
            let a = matrix! {
                3 5 0;
                2 -1 -7;
                6 -1 5;
            };

            assert_eq!(-12., a.minor(0, 0));
            assert_eq!(-12., a.cofactor(0, 0));
            assert_eq!(25., a.minor(1, 0));
            assert_eq!(-25., a.cofactor(1, 0));
        }

        #[test]
        fn calculating_determinant_of_3x3() {
            let a = matrix! {
                1 2 6;
                -5 8 -4;
                2 6 4;
            };

            assert_eq!(56., a.cofactor(0, 0));
            assert_eq!(12., a.cofactor(0, 1));
            assert_eq!(-46., a.cofactor(0, 2));
            assert_eq!(-196., a.determinant());
        }

        #[test]
        fn testing_an_invertible_matrix_for_invertibility() {
            let a = matrix! {
                 6  4  4  4;
                 5  5  7  6;
                 4 -9  3 -7;
                 9  1  7 -6;
            };

            assert!(a.invertible());
        }

        #[test]
        fn testing_an_noninvertible_matrix_for_invertibility() {
            let a = matrix! {
                -4  4 -2 -3;
                 9  6  2  6;
                 0 -5  1 -5;
                 0  0  0  0;
            };

            assert!(!a.invertible());
        }

        #[test]
        fn calculating_inverse_of_matrix() {
            let a = matrix! {
                -5 2 6 -8;
                1 -5 1 8;
                7 7 -6 -7;
                1 -3 7 4;
            };

            let b = a.invert();
            let expected = matrix! {
                 0.21805   0.45113   0.24060 -0.04511 ;
                -0.80827  -1.45677  -0.44361  0.52068 ;
                -0.07895  -0.22368  -0.05263  0.19737 ;
                -0.52256  -0.81391  -0.30075  0.30639 ;
            };

            assert_eq!(a.determinant(), 532.);
            assert_eq!(a.cofactor(2, 3), -160.);
            assert_eq!(b[3][2], -160. / 532.);
            assert_eq!(a.cofactor(3, 2), 105.);
            assert_eq!(b[2][3], 105. / 532.);

            assert_eq!(b, expected);
        }

        #[test]
        fn calculating_inverse_of_another_matrix() {
            let a = matrix! {
                 8 -5  9  2;
                 7  5  6  1;
                -6  0  9  6;
                -3  0 -9 -4;
            };

            let expected = matrix! {
                 -0.15385  -0.15385  -0.28205  -0.53846 ;
                 -0.07692   0.12308   0.02564   0.03077 ;
                  0.35897   0.35897   0.43590   0.92308 ;
                 -0.69231  -0.69231  -0.76923  -1.92308 ;
            };

            assert_eq!(a.invert(), expected);
        }

        #[test]
        fn calculating_inverse_of_another_another_matrix() {
            let a = matrix! {
                 9  3  0  9 ;
                -5 -2 -6 -3 ;
                -4  9  6  4 ;
                -7  6  6  2 ;
            };

            let expected = matrix! {
                -0.04074  -0.07778   0.14444  -0.22222 ;
                -0.07778   0.03333   0.36667  -0.33333 ;
                -0.02901  -0.14630  -0.10926   0.12963 ;
                 0.17778   0.06667  -0.26667   0.33333 ;
            };

            assert_eq!(a.invert(), expected);
        }

        #[test]
        fn multiplying_a_product_by_its_inverse() {
            let a = matrix! {
                 3 -9  7  3;
                 3 -8  2 -9;
                -4  4  4  1;
                -6  5 -1  1;
            };

            let b = matrix! {
                 8  2  2  2;
                 3 -1  7  0;
                 7  0  5  4;
                 6 -2  0  5;
            };

            let c = a * b;

            assert_eq!(c * b.invert(), a);
        }
    }

    #[cfg(test)]
    mod transformations {
        use super::*;
        use apolaki_tuple::{point, vector};
        use std::f64::consts::PI;

        #[test]
        fn multiplying_by_a_translation_matrix() {
            let transform = BaseMatrix::identity().translate(5, -3, 2);
            let p = point(-3, 4, 5);

            assert_eq!(point(2, 1, 7), transform * p);
        }

        #[test]
        fn multiplying_by_the_inverse_of_translation_matrix() {
            let transform = BaseMatrix::identity().translate(5, -3, 2);
            let inv = transform.invert();
            let p = point(-3, 4, 5);

            assert_eq!(point(-8, 7, 3), inv * p);
        }

        #[test]
        fn translation_does_not_vectors() {
            let transform = BaseMatrix::identity().translate(5, -3, 2);

            let v = vector(-3, 4, 5);
            assert_eq!(v, transform * v);
        }

        #[test]
        fn scaling_matrix_to_a_point() {
            let transform = BaseMatrix::identity().scale(2, 3, 4);

            let p = point(-4, 6, 8);
            assert_eq!(point(-8, 18, 32), transform * p);
        }

        #[test]
        fn scaling_matrivector() {
            let transform = BaseMatrix::identity().scale(2, 3, 4);

            let v = vector(-4, 6, 8);
            assert_eq!(vector(-8, 18, 32), transform * v);
        }

        #[test]
        fn multiplying_by_inverse_of_scaling_matrix() {
            let transform = BaseMatrix::identity().scale(2, 3, 4);

            let inv = transform.invert();
            let v = vector(-4, 6, 8);

            assert_eq!(vector(-2, 2, 2), inv * v);
        }

        #[test]
        fn reflection_is_scaling_by_negative_value() {
            let transform = BaseMatrix::identity().scale(-1, 1, 1);

            let p = vector(2, 3, 4);

            assert_eq!(vector(-2, 3, 4), transform * p);
        }

        #[test]
        fn rotate_a_point_around_x() {
            let p = point(0, 1, 0);

            let half_quarter = BaseMatrix::identity().rotate_x(PI / 4.);

            let full_quarter = BaseMatrix::identity().rotate_x(PI / 2.);

            assert_eq!(
                point(0, 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.),
                half_quarter * p
            );
            assert_eq!(point(0, 0, 1), full_quarter * p);
        }

        #[test]
        fn inverse_of_x_rotatation_rotates_in_opposite_dir() {
            let p = point(0, 1, 0);

            let half_quarter = BaseMatrix::identity().rotate_x(PI / 4.);
            let inv = half_quarter.invert();
            assert_eq!(
                point(0, 2_f64.sqrt() / 2., -(2_f64.sqrt()) / 2.),
                inv * p
            );
        }

        #[test]
        fn rotate_a_point_around_y() {
            let p = point(0, 0, 1);

            let half_quarter = BaseMatrix::identity().rotate_y(PI / 4.);

            let full_quarter = BaseMatrix::identity().rotate_y(PI / 2.);

            assert_eq!(
                point(2_f64.sqrt() / 2., 0., 2_f64.sqrt() / 2.),
                half_quarter * p
            );
            assert_eq!(point(1, 0, 0), full_quarter * p);
        }

        #[test]
        fn rotate_a_point_around_z() {
            let p = point(0, 1, 0);

            let half_quarter = BaseMatrix::identity().rotate_z(PI / 4.);

            let full_quarter = BaseMatrix::identity().rotate_z(PI / 2.);

            assert_eq!(
                point(-(2_f64.sqrt()) / 2., 2_f64.sqrt() / 2., 0),
                half_quarter * p
            );
            assert_eq!(point(-1, 0, 0), full_quarter * p);
        }

        #[test]
        fn shearing_transform_moves_x_in_proportion_to_y() {
            let transform = BaseMatrix::identity().shear(1, 0, 0, 0, 0, 0);

            let p = point(2, 3, 4);
            assert_eq!(point(5, 3, 4), transform * p);
        }

        #[test]
        fn shearing_transform_moves_x_in_proportion_to_z() {
            let transform = BaseMatrix::identity().shear(0, 1, 0, 0, 0, 0);

            let p = point(2, 3, 4);
            assert_eq!(point(6, 3, 4), transform * p);
        }

        #[test]
        fn shearing_transform_moves_y_in_proportion_to_x() {
            let transform = BaseMatrix::identity().shear(0, 0, 1, 0, 0, 0);

            let p = point(2, 3, 4);
            assert_eq!(point(2, 5, 4), transform * p);
        }

        #[test]
        fn shearing_transform_moves_y_in_proportion_to_z() {
            let transform = BaseMatrix::identity().shear(0, 0, 0, 1, 0, 0);

            let p = point(2, 3, 4);
            assert_eq!(point(2, 7, 4), transform * p);
        }

        #[test]
        fn shearing_transform_moves_z_in_proportion_to_x() {
            let transform = BaseMatrix::identity().shear(0, 0, 0, 0, 1, 0);

            let p = point(2, 3, 4);
            assert_eq!(point(2, 3, 6), transform * p);
        }

        #[test]
        fn shearing_transform_moves_z_in_proportion_to_y() {
            let transform = BaseMatrix::identity().shear(0, 0, 0, 0, 0, 1);

            let p = point(2, 3, 4);
            assert_eq!(point(2, 3, 7), transform * p);
        }

        #[test]
        fn individual_transformation_are_applied_in_sequence() {
            let p = point(1, 0, 1);
            let a = BaseMatrix::identity().rotate_x(PI / 2.);
            let b = BaseMatrix::identity().scale(5, 5, 5);
            let c = BaseMatrix::identity().translate(10, 5, 7);

            let p2 = a * p;

            assert_eq!(point(1, -1, 0), p2);

            let p3 = b * p2;

            assert_eq!(point(5, -5, 0), p3);

            let p4 = c * p3;

            assert_eq!(point(15, 0, 7), p4);
        }

        #[test]
        fn chained_transform_must_be_applied_in_reverse() {
            let p = point(1, 0, 1);
            let a = BaseMatrix::identity().rotate_x(PI / 2.);
            let b = BaseMatrix::identity().scale(5, 5, 5);
            let c = BaseMatrix::identity().translate(10, 5, 7);

            let t = c * b * a;
            assert_eq!(point(15, 0, 7), t * p);
        }

        #[test]
        fn chained_transform_must_be_applied_in_reverse_fluent() {
            let p = point(1, 0, 1);

            let t = BaseMatrix::identity()
                .rotate_x(PI / 2.)
                .scale(5, 5, 5)
                .translate(10, 5, 7);
            assert_eq!(point(15, 0, 7), t * p);
        }
    }
}
