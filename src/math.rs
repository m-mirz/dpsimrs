extern crate nalgebra as na;

use std::fmt::Display;

use na::{ArrayStorage, Complex, Dynamic, Matrix, Matrix2x1, OMatrix, LU, U2};
use numpy::ToPyArray;
use pyo3::prelude::*;

#[derive(Clone, Debug)]
pub struct DMatrixf64(pub OMatrix<f64, Dynamic, Dynamic>);

#[derive(Clone, Debug)]
pub struct DMatrixc64(pub OMatrix<Complex<f64>, Dynamic, Dynamic>);

#[derive(Clone, Debug)]
pub struct Matrix2x2f64(pub Matrix<f64, U2, U2, ArrayStorage<f64, 2, 2>>);

#[derive(Clone, Debug)]
pub struct Matrix2x2c64(pub Matrix<Complex<f64>, U2, U2, ArrayStorage<Complex<f64>, 2, 2>>);

#[derive(Clone, Debug)]
pub struct Matrix2x1f64(pub Matrix2x1<f64>);

#[derive(Clone, Debug)]
pub struct Matrix2x1c64(pub Matrix2x1<Complex<f64>>);

impl IntoPy<PyObject> for DMatrixf64 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.to_pyarray(py).to_object(py)
    }
}

#[allow(dead_code)]
impl DMatrixf64 {
    pub fn zeros(nrows: usize, ncols: usize) -> Self {
        Self(OMatrix::<f64, Dynamic, Dynamic>::zeros(nrows, ncols))
    }

    pub fn lu(self) -> LU<f64, Dynamic, Dynamic> {
        self.0.lu()
    }
}

impl Display for DMatrixf64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoPy<PyObject> for DMatrixc64 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.to_pyarray(py).to_object(py)
    }
}

impl DMatrixc64 {
    pub fn zeros(nrows: usize, ncols: usize) -> Self {
        Self(OMatrix::<Complex<f64>, Dynamic, Dynamic>::zeros(
            nrows, ncols,
        ))
    }

    pub fn lu(self) -> LU<Complex<f64>, Dynamic, Dynamic> {
        self.0.lu()
    }
}

impl Display for DMatrixc64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[allow(dead_code)]
impl Matrix2x2f64 {
    pub fn new(m11: f64, m12: f64, m21: f64, m22: f64) -> Self {
        Self(Matrix::<f64, U2, U2, ArrayStorage<f64, 2, 2>>::new(
            m11, m12, m21, m22,
        ))
    }

    pub fn zeros() -> Self {
        Self(Matrix::<f64, U2, U2, ArrayStorage<f64, 2, 2>>::zeros())
    }
}

impl IntoPy<PyObject> for Matrix2x2f64 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.to_pyarray(py).to_object(py)
    }
}

impl Display for Matrix2x2f64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Matrix2x2c64 {
    pub fn new(m11: Complex<f64>, m12: Complex<f64>, m21: Complex<f64>, m22: Complex<f64>) -> Self {
        Self(Matrix::<
            Complex<f64>,
            U2,
            U2,
            ArrayStorage<Complex<f64>, 2, 2>,
        >::new(m11, m12, m21, m22))
    }

    pub fn zeros() -> Self {
        Self(Matrix::<
            Complex<f64>,
            U2,
            U2,
            ArrayStorage<Complex<f64>, 2, 2>,
        >::zeros())
    }
}

impl IntoPy<PyObject> for Matrix2x2c64 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.to_pyarray(py).to_object(py)
    }
}

impl Display for Matrix2x2c64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[allow(dead_code)]
impl Matrix2x1f64 {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Matrix2x1::<f64>::new(x, y))
    }

    pub fn zeros() -> Self {
        Self(Matrix2x1::<f64>::zeros())
    }
}

impl Display for Matrix2x1f64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoPy<PyObject> for Matrix2x1f64 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.to_pyarray(py).to_object(py)
    }
}

impl Matrix2x1c64 {
    pub fn new(x: Complex<f64>, y: Complex<f64>) -> Self {
        Self(Matrix2x1::<Complex<f64>>::new(x, y))
    }

    pub fn zeros() -> Self {
        Self(Matrix2x1::<Complex<f64>>::zeros())
    }
}

impl Display for Matrix2x1c64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoPy<PyObject> for Matrix2x1c64 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.to_pyarray(py).to_object(py)
    }
}
