use crate::math::{Matrix2x2f64, Matrix2x1f64};

pub const GROUND: usize = usize::MAX;

#[repr(usize)]
#[derive(PartialEq, Debug)]
pub enum NodeType {
    Network,
    Ground = GROUND,
}

#[derive(Debug, PartialEq)]
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
pub enum ComponentType<'a> {
    Resistor(Resistor<'a>),
    CurrentSource(CurrentSource<'a>)
}

#[derive(Debug)]
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
pub struct Node<'a> {
    pub params: &'a NodeParams,
    pub index: usize,
}

#[derive(Debug)]
pub struct Resistor<'a> {
    pub params: &'a ResistorParams,
    pub stamp: Matrix2x2f64,
    pub node_1_idx: usize,
    pub node_2_idx: usize,
}

impl Resistor<'_> {
    pub fn calculate_stamp(&mut self) {
        let conductance = 1.0 / self.params.resistance;
        self.stamp = Matrix2x2f64::new( conductance, -conductance,
                                        -conductance, conductance);
    }
}

#[derive(Debug)]
pub struct CurrentSource<'a> {
    pub params: &'a CurrentSourceParams,
    pub stamp: Matrix2x1f64,
    pub node_1_idx: usize,
    pub node_2_idx: usize,
}

impl CurrentSource<'_> {
    pub fn calculate_stamp(&mut self) {
        self.stamp = Matrix2x1f64::new(self.params.set_point,
                                      -self.params.set_point);
    }
}

#[derive(Debug)]
pub struct NetworkState<'a> {
    pub nodes: Vec<Node<'a>>,
    pub comps: Vec<ComponentType<'a>>,
}

impl<'a> NetworkState<'a> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            comps: Vec::new(),
        }
    }

    pub fn generate_sim_objects(&mut self, net_params: &'a NetworkParams) {
        for (idx, node_params) in net_params.nodes.iter().enumerate() {
            if node_params.node_type != NodeType::Ground {
                self.nodes.push(Node{
                    params: node_params,
                    index: idx,
                });
            }
        }

        for comp_param in net_params.comps.iter() {
            match comp_param {
                ComponentParams::Resistor(res_params) => {
                    self.comps.push(ComponentType::Resistor(Resistor{
                        params: res_params,
                        stamp: Matrix2x2f64::zeros(),
                        node_1_idx: res_params.node_1,
                        node_2_idx: res_params.node_2,
                   }));
                }
                ComponentParams::CurrentSource(src_params) => {
                    self.comps.push(ComponentType::CurrentSource(CurrentSource{
                        params: src_params,
                        stamp: Matrix2x1f64::zeros(),
                        node_1_idx: src_params.node_1,
                        node_2_idx: src_params.node_2,
                    }));
                }
            }
        }
    }
}
