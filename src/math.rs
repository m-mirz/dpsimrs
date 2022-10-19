extern crate nalgebra as na;

use na::{
    OMatrix, Dynamic, Matrix, ArrayStorage, U2, Matrix2x1,
    //Complex,
};

pub type DMatrixf64 = OMatrix<f64, Dynamic, Dynamic>;
//pub type DMatrixc64 = OMatrix<Complex<f64>, Dynamic, Dynamic>;
pub type Matrix2x2f64 = Matrix<f64, U2, U2, ArrayStorage<f64, 2, 2>>;
pub type Matrix2x1f64 = Matrix2x1<f64>;
