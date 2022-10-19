use std::ops::{AddAssign};
use crate::math::{DMatrixf64, Matrix2x2f64, Matrix2x1f64};
use crate::components::{Network, ComponentType, Node, ComponentParams, Resistor, CurrentSource, NodeType};


pub struct Simulation {
    pub net_matrix: DMatrixf64,
    pub rhs_vector: DMatrixf64,
    pub net_size: usize,
}

impl Simulation{
    pub fn new(net: &mut Network) -> Self {
        let node_number = net.node_params.len()-1;

        Self {
            net_matrix: DMatrixf64::zeros(node_number, node_number),
            rhs_vector: DMatrixf64::zeros(node_number, 1),
            net_size: node_number,
        }
    }

    pub fn generate_sim_objects(&self, net: &mut Network) {
        for (idx, node_param) in net.node_params.iter().enumerate() {
            if node_param.node_type == NodeType::Ground {
                net.nodes.push(Node{
                    params: node_param,
                    index: idx,
                });
            } else {
                net.nodes.push(Node{
                    params: node_param,
                    index: idx-1,
                });
            }
        }

        for comp_param in net.comp_params.iter() {
            match comp_param {
                ComponentParams::Resistor(res_params) => {
                    let n1 = net.node_params.iter().position(|&x| x == res_params.node_1).expect("Could not map node!");
                    let n2 = net.node_params.iter().position(|&x| x == res_params.node_2).expect("Could not map node!");

                    net.comps.push(ComponentType::Resistor(Resistor{
                        params: res_params,
                        stamp: Matrix2x2f64::zeros(),
                        node_1: net.nodes[n1].index,
                        node_2: net.nodes[n2].index,
                   }));
                }
                ComponentParams::CurrentSource(src_params) => {
                    let n1 = net.node_params.iter().position(|&x| x == src_params.node_1).expect("Could not map node!");
                    let n2 = net.node_params.iter().position(|&x| x == src_params.node_2).expect("Could not map node!");

                    net.comps.push(ComponentType::CurrentSource(CurrentSource{
                        params: src_params,
                        stamp: Matrix2x1f64::zeros(),
                        node_1: net.nodes[n1].index,
                        node_2: net.nodes[n2].index,
                    }));
                }
            }
        }
    }

    pub fn stamp(&mut self, net: &mut Network) {
        for comp in net.comps.iter_mut() {
            match comp {
                ComponentType::Resistor(res) => {
                    res.calculate_stamp();
                }
                ComponentType::CurrentSource(src) => {
                    src.calculate_stamp();
                }
            }
        }

        for comp in net.comps.iter() {
            match comp {
                ComponentType::Resistor(res) => {
                    self.stamp_branch(res);
                }
                ComponentType::CurrentSource(src) => {
                    self.stamp_source(src);
                }
            }
        }
    }

    pub fn stamp_branch(&mut self, branch: &Resistor) {
        if branch.params.node_1.node_type != NodeType::Ground {
            self.net_matrix[(branch.node_1, branch.node_1)].add_assign(branch.stamp[(0,0)]);
        }

        if branch.params.node_2.node_type != NodeType::Ground {
            self.net_matrix[(branch.node_2, branch.node_2)].add_assign(branch.stamp[(1,1)]);
        }

        if branch.params.node_1.node_type != NodeType::Ground && branch.params.node_2.node_type != NodeType::Ground {
            self.net_matrix[(branch.node_1, branch.node_2)].add_assign(branch.stamp[(0,1)]);
            self.net_matrix[(branch.node_2, branch.node_1)].add_assign(branch.stamp[(1,0)]);
        }
    }

    pub fn stamp_source(&mut self, source: &CurrentSource, ) {
        if source.params.node_1.node_type != NodeType::Ground {
            self.rhs_vector[(source.node_1, 0)] = -source.params.set_point;
        }

        if source.params.node_2.node_type != NodeType::Ground {
            self.rhs_vector[(source.node_2, 0)] = source.params.set_point;
        }
    }
}
