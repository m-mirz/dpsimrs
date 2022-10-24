use crate::math::{Matrix2x1f64, Matrix2x2f64};

pub const GROUND: usize = usize::MAX;

#[repr(usize)]
#[derive(PartialEq, Eq, Debug)]
pub enum NodeType {
    Network,
    Ground = GROUND,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NodeParams {
    pub id: String,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum ComponentParams {
    Resistor(ResistorParams),
    CurrentSource(CurrentSourceParams),
}

#[derive(Debug)]
pub struct ResistorParams {
    pub id: String,
    pub resistance: f64,
    pub node_1: usize,
    pub node_2: usize,
}

#[derive(Debug)]
pub struct CurrentSourceParams {
    pub id: String,
    pub set_point: f64,
    pub node_1: usize,
    pub node_2: usize,
}

#[derive(Debug)]
pub enum ComponentType {
    Resistor(Resistor),
    CurrentSource(CurrentSource),
}

#[derive(Debug, Default)]
pub struct NetworkParams {
    pub nodes: Vec<NodeParams>,
    pub comps: Vec<ComponentParams>,
}

impl NetworkParams {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            comps: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_param: NodeParams) -> usize {
        if node_param.node_type == NodeType::Ground {
            // maximum value of usize is reserved for ground node
            return NodeType::Ground as usize;
        } else {
            self.nodes.push(node_param);
            return self.nodes.len() - 1;
        }
    }

    pub fn add_component(&mut self, comp_param: ComponentParams) {
        self.comps.push(comp_param);
    }
}

#[derive(Debug)]
pub struct Node {
    pub params: NodeParams,
    pub index: usize,
}

#[derive(Debug)]
pub struct Resistor {
    pub params: ResistorParams,
    pub stamp: Matrix2x2f64,
    pub node_1_idx: usize,
    pub node_2_idx: usize,
}

impl Resistor {
    pub fn calculate_stamp(&mut self) {
        let conductance = 1.0 / self.params.resistance;
        self.stamp = Matrix2x2f64::new(conductance, -conductance, -conductance, conductance);
    }
}

#[derive(Debug)]
pub struct CurrentSource {
    pub params: CurrentSourceParams,
    pub stamp: Matrix2x1f64,
    pub node_1_idx: usize,
    pub node_2_idx: usize,
}

impl CurrentSource {
    pub fn calculate_stamp(&mut self) {
        self.stamp = Matrix2x1f64::new(self.params.set_point, -self.params.set_point);
    }
}

#[derive(Debug)]
pub struct NetworkState {
    pub nodes: Vec<Node>,
    pub comps: Vec<ComponentType>,
}

impl NetworkState {
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
                ComponentParams::Resistor(res_params) => {
                    self.comps.push(ComponentType::Resistor(Resistor {
                        stamp: Matrix2x2f64::zeros(),
                        node_1_idx: res_params.node_1,
                        node_2_idx: res_params.node_2,
                        params: res_params,
                    }));
                }
                ComponentParams::CurrentSource(src_params) => {
                    self.comps.push(ComponentType::CurrentSource(CurrentSource {
                        stamp: Matrix2x1f64::zeros(),
                        node_1_idx: src_params.node_1,
                        node_2_idx: src_params.node_2,
                        params: src_params,
                    }));
                }
            }
        }
    }
}
