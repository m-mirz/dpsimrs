use dpsimrs::models::{
    ComponentParams, CurrentSourceParams, LineParams, NetworkParams, NetworkState, NodeParams,
    NodeType,
};
use dpsimrs::simulation::Simulation;
use nalgebra::Complex;

#[test]
fn res_csrc_circuit() {
    let mut net_params = NetworkParams::new();

    let grd = net_params.add_node(NodeParams {
        id: "GRD".to_string(),
        node_type: NodeType::Ground,
    });
    let n1 = net_params.add_node(NodeParams {
        id: "N1".to_string(),
        node_type: NodeType::Network,
    });

    let r1 = ComponentParams::Line(LineParams {
        id: String::from("r1"),
        resistance: 1.0,
        reactance: 0.0,
        node_1: grd,
        node_2: n1,
    });

    net_params.add_component(r1);

    let c1 = ComponentParams::CurrentSource(CurrentSourceParams {
        id: String::from("c1"),
        set_point_re: 5.0,
        set_point_im: 0.0,
        node_1: n1,
        node_2: grd,
    });

    net_params.add_component(c1);
    println!("{:?}", net_params);

    let mut net_state = NetworkState::new();
    net_state.generate_sim_objects(net_params);

    println!("Nodes: \n{:?}\n", net_state.nodes);
    println!("Components: \n{:?}\n", net_state.comps);

    let mut sim = Simulation::new(&mut net_state);
    println!("Network Matrix: \n{}", sim.net_matrix);
    println!("Right-hand vector: \n{}", sim.rhs_vector);

    sim.stamp(&mut net_state);
    println!("Network Matrix: \n{}", sim.net_matrix);
    println!("Right-hand vector: \n{}", sim.rhs_vector);

    let decomp = sim.net_matrix.lu();
    let lhs_vector = decomp.solve(&sim.rhs_vector.0).expect("msg");
    println!("Left-hand vector: \n{}", lhs_vector);

    assert_eq!(lhs_vector[0], Complex::new(5.0, 0.0));
}
