use crate::math::{Matrix2x2f64, Matrix2x1f64};

#[derive(PartialEq, Debug)]
pub enum NodeType {
    Network,
    Ground,
}

#[derive(Debug, PartialEq)]
pub struct NodeParams {
    pub id: String,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum ComponentParams<'a> {
    Resistor(ResistorParams<'a>),
    CurrentSource(CurrentSourceParams<'a>),
}

#[derive(Debug)]
pub struct ResistorParams<'a> {
    pub id: String,
    pub resistance: f64,
    pub node_1: &'a NodeParams,
    pub node_2: &'a NodeParams,
}

#[derive(Debug)]
pub struct CurrentSourceParams<'a> {
    pub id: String,
    pub set_point: f64,
    pub node_1: &'a NodeParams,
    pub node_2: &'a NodeParams,
}

#[derive(Debug)]
pub enum ComponentType<'a> {
    Resistor(Resistor<'a>),
    CurrentSource(CurrentSource<'a>)
}

#[derive(Debug)]
pub struct Network<'a> {
    pub node_params: Vec<&'a NodeParams>,
    pub comp_params: Vec<&'a ComponentParams<'a>>,
    pub nodes: Vec<Node<'a>>,
    pub comps: Vec<ComponentType<'a>>,
}

impl<'a> Network<'a> {
    pub fn new() -> Self {
        Self {
            node_params: Vec::new(),
            comp_params: Vec::new(),
            nodes: Vec::new(),
            comps: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_param: &'a NodeParams) {
        self.node_params.push(node_param);

    }

    pub fn add_component(&mut self, comp_param: &'a ComponentParams) {
        self.comp_params.push(comp_param);
    }
}

#[derive(Debug)]
pub struct Node<'a> {
    pub params: &'a NodeParams,
    pub index: usize,
}

#[derive(Debug)]
pub struct Resistor<'a> {
    pub params: &'a ResistorParams<'a>,
    pub stamp: Matrix2x2f64,
    pub node_1: usize,
    pub node_2: usize,
}

impl Resistor<'_> {
    pub fn calculate_stamp(&mut self) {
        let conductance = 1.0 / self.params.resistance;
        self.stamp = Matrix2x2f64::new( conductance, conductance,
                                        conductance, conductance);
    }
}

#[derive(Debug)]
pub struct CurrentSource<'a> {
    pub params: &'a CurrentSourceParams<'a>,
    pub stamp: Matrix2x1f64,
    pub node_1: usize,
    pub node_2: usize,
}

impl CurrentSource<'_> {
    pub fn calculate_stamp(&mut self) {
        self.stamp = Matrix2x1f64::new(self.params.set_point,
                                      -self.params.set_point);
    }
}
