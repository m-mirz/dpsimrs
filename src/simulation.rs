use crate::math::{DMatrixc64, Matrix2x1c64, Matrix2x2c64};
use crate::models::{ComponentType, NetworkState, GROUND};
use ndarray::AssignElem;
use pyo3::prelude::*;
use std::ops::AddAssign;

#[pyclass]
pub struct Simulation {
    #[pyo3(get)]
    pub net_matrix: DMatrixc64,
    #[pyo3(get)]
    pub rhs_vector: DMatrixc64,
    pub net_size: usize,
}

#[pymethods]
impl Simulation {
    #[new]
    pub fn new(net: &mut NetworkState) -> Self {
        let node_number = net.nodes.len();

        Self {
            net_matrix: DMatrixc64::zeros(node_number, node_number),
            rhs_vector: DMatrixc64::zeros(node_number, 1),
            net_size: node_number,
        }
    }

    pub fn stamp(&mut self, net: &mut NetworkState) {
        for comp in net.comps.iter_mut() {
            match comp {
                ComponentType::Line(res) => {
                    res.calculate_stamp();
                    self.stamp_branch(&res.stamp, res.params.node_1, res.params.node_2);
                }
                ComponentType::CurrentSource(src) => {
                    src.calculate_stamp();
                    self.stamp_node_injection(&src.stamp, src.params.node_1, src.params.node_2);
                }
            }
        }
    }

    pub fn solve(&self) -> DMatrixc64 {
        DMatrixc64(
            self.net_matrix
                .clone()
                .lu()
                .solve(&self.rhs_vector.0)
                .expect("Solve matrix-vector equation system"),
        )
    }
}

impl Simulation {
    pub fn stamp_branch(&mut self, stamp: &Matrix2x2c64, node_1: usize, node_2: usize) {
        if node_1 != GROUND {
            self.net_matrix.0[(node_1, node_1)].add_assign(stamp.0[(0, 0)]);
        }

        if node_2 != GROUND {
            self.net_matrix.0[(node_2, node_2)].add_assign(stamp.0[(1, 1)]);
        }

        if node_1 != GROUND && node_2 != GROUND {
            self.net_matrix.0[(node_1, node_2)].add_assign(stamp.0[(0, 1)]);
            self.net_matrix.0[(node_2, node_1)].add_assign(stamp.0[(1, 0)]);
        }
    }

    pub fn stamp_node_injection(&mut self, stamp: &Matrix2x1c64, node_1: usize, node_2: usize) {
        if node_1 != GROUND {
            self.rhs_vector.0[(node_1, 0)].assign_elem(stamp.0[(0, 0)]);
        }

        if node_2 != GROUND {
            self.rhs_vector.0[(node_2, 0)].assign_elem(stamp.0[(1, 0)]);
        }
    }
}
