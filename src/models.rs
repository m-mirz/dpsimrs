use crate::math::{Matrix2x1c64, Matrix2x2c64};
use nalgebra::Complex;
use pyo3::prelude::*;

pub const GROUND: usize = usize::MAX;

// suppress clippy warning, false positive?
// https://github.com/rust-lang/rust-clippy/issues/8043
#[allow(clippy::enum_clike_unportable_variant)]
#[repr(usize)]
#[pyclass]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NodeType {
    Network,
    PQbus,
    PVbus,
    RefBus,
    Ground = GROUND
}

#[pyclass]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NodeParams {
    pub id: String,
    pub node_type: NodeType,
}

#[derive(Debug)]
#[pyclass]
pub struct Node {
    pub params: NodeParams,
    pub index: usize,
}

#[pyclass]
#[derive(Debug, Default, Clone)]
pub struct NetworkParams {
    pub nodes: Vec<NodeParams>,
    pub comps: Vec<ComponentParams>,
}

#[pyclass]
#[derive(Debug, Default)]
pub struct NetworkState {
    pub nodes: Vec<Node>,
    pub comps: Vec<ComponentType>,
}

#[derive(Debug, Clone, FromPyObject)]
pub enum ComponentParams {
    #[pyo3(transparent)]
    CurrentSource(CurrentSourceParams),
    #[pyo3(transparent)]
    Line(LineParams),
    #[pyo3(transparent)]
    PQSource(PQSourceParams)
}

#[derive(Debug, FromPyObject)]
pub enum ComponentType {
    #[pyo3(transparent)]
    Line(Line),
    #[pyo3(transparent)]
    CurrentSource(CurrentSource),
    #[pyo3(transparent)]
    PQSource(PQSource)
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct LineParams {
    pub id: String,
    pub resistance: f64,
    pub reactance: f64,
    pub node_1: usize,
    pub node_2: usize,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Line {
    pub params: LineParams,
    pub stamp: Matrix2x2c64,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CurrentSourceParams {
    pub id: String,
    pub set_point: Complex<f64>,
    pub node_1: usize,
    pub node_2: usize,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CurrentSource {
    pub params: CurrentSourceParams,
    pub stamp: Matrix2x1c64,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PQSourceParams {
    pub id: String,
    pub set_point: Complex<f64>,
    pub node: usize,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PQSource {
    pub params: PQSourceParams,
    pub stamp: Matrix2x1c64,
}

#[pymethods]
impl NodeParams {
    #[new]
    pub fn new(id: String, node_type: NodeType) -> Self {
        NodeParams { id, node_type }
    }
}

#[pymethods]
impl NetworkParams {
    #[new]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            comps: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_param: NodeParams) -> usize {
        if node_param.node_type == NodeType::Ground {
            // maximum value of usize is reserved for ground node
            NodeType::Ground as usize
        } else {
            self.nodes.push(node_param);
            self.nodes.len() - 1
        }
    }

    pub fn add_component(&mut self, comp_param: ComponentParams) {
        self.comps.push(comp_param);
    }
}

#[pymethods]
impl NetworkState {
    #[new]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            comps: Vec::new(),
        }
    }

    pub fn generate_sim_objects(&mut self, net_params: NetworkParams) {
        for (idx, node_params) in net_params.nodes.into_iter().enumerate() {
            if node_params.node_type != NodeType::Ground {
                self.nodes.push(Node {
                    params: node_params,
                    index: idx,
                });
            }
        }

        for comp_param in net_params.comps.into_iter() {
            match comp_param {
                ComponentParams::Line(res_params) => {
                    self.comps.push(ComponentType::Line(Line {
                        stamp: Matrix2x2c64::zeros(),
                        params: res_params,
                    }));
                }
                ComponentParams::CurrentSource(src_params) => {
                    self.comps.push(ComponentType::CurrentSource(CurrentSource {
                        stamp: Matrix2x1c64::zeros(),
                        params: src_params,
                    }));
                }
                ComponentParams::PQSource(src_params) => {
                    self.comps.push(ComponentType::PQSource(PQSource {
                        stamp: Matrix2x1c64::zeros(),
                        params: src_params,
                    }));
                }
            }
        }
    }
}

#[pymethods]
impl LineParams {
    #[new]
    pub fn new(id: String, resistance: f64, reactance: f64, node_1: usize, node_2: usize) -> Self {
        LineParams {
            id,
            resistance,
            reactance,
            node_1,
            node_2,
        }
    }
}

impl Line {
    pub fn calculate_stamp(&mut self) {
        let admittance = 1.0 / Complex::new(self.params.resistance, self.params.reactance);
        self.stamp = Matrix2x2c64::new(admittance, -admittance, -admittance, admittance);
    }
}

#[pymethods]
impl CurrentSourceParams {
    #[new]
    pub fn new(id: String, set_point: Complex<f64>, node_1: usize, node_2: usize) -> Self {
        CurrentSourceParams {
            id,
            set_point,
            node_1,
            node_2,
        }
    }
}

impl CurrentSource {
    pub fn calculate_stamp(&mut self) {
        self.stamp = Matrix2x1c64::new(self.params.set_point, -self.params.set_point);
    }
}

impl PQSource {
    pub fn calculate_stamp(&mut self) {
        self.stamp = Matrix2x1c64::new(self.params.set_point, -self.params.set_point);
    }
}
