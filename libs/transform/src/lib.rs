mod transform {
    use apolaki_matrix::BaseMatrix;

    pub trait Transform: Sized {
        fn transform(&self, m: BaseMatrix<4>) -> Self;
    }
}

pub use transform::*;
