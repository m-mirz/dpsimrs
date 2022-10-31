use std::ops::AddAssign;

use crate::math::DMatrixf64;
use crate::models::{ComponentType, CurrentSource, NetworkState, Resistor, GROUND};
use pyo3::prelude::*;

#[pyclass]
pub struct Simulation {
    #[pyo3(get)]
    pub net_matrix: DMatrixf64,
    #[pyo3(get)]
    pub rhs_vector: DMatrixf64,
    pub net_size: usize,
}

#[pymethods]
impl Simulation {

    #[new]
    pub fn new(net: &mut NetworkState) -> Self {
        let node_number = net.nodes.len();

        Self {
            net_matrix: DMatrixf64::zeros(node_number, node_number),
            rhs_vector: DMatrixf64::zeros(node_number, 1),
            net_size: node_number,
        }
    }

    pub fn stamp(&mut self, net: &mut NetworkState) {
        for comp in net.comps.iter_mut() {
            match comp {
                ComponentType::Resistor(res) => {
                    res.calculate_stamp();
                    self.stamp_branch(res);
                }
                ComponentType::CurrentSource(src) => {
                    src.calculate_stamp();
                    self.stamp_source(src);
                }
            }
        }
    }

    pub fn stamp_branch(&mut self, branch: &Resistor) {
        if branch.node_1_idx != GROUND {
            self.net_matrix.0[(branch.node_1_idx, branch.node_1_idx)]
                .add_assign(branch.stamp.0[(0, 0)]);
        }

        if branch.node_2_idx != GROUND {
            self.net_matrix.0[(branch.node_2_idx, branch.node_2_idx)]
                .add_assign(branch.stamp.0[(1, 1)]);
        }

        if branch.node_1_idx != GROUND && branch.node_2_idx != GROUND {
            self.net_matrix.0[(branch.node_1_idx, branch.node_2_idx)]
                .add_assign(branch.stamp.0[(0, 1)]);
            self.net_matrix.0[(branch.node_2_idx, branch.node_1_idx)]
                .add_assign(branch.stamp.0[(1, 0)]);
        }
    }

    pub fn stamp_source(&mut self, source: &CurrentSource) {
        if source.node_1_idx != GROUND {
            self.rhs_vector.0[(source.node_1_idx, 0)] = -source.params.set_point;
        }

        if source.node_2_idx != GROUND {
            self.rhs_vector.0[(source.node_2_idx, 0)] = source.params.set_point;
        }
    }

    pub fn solve(&self) -> DMatrixf64 {
        DMatrixf64(self.net_matrix.clone().lu().solve(&self.rhs_vector.0).expect("Solve matrix-vector equation system"))
    }
}
