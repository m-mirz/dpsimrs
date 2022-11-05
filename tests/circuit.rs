use dpsimrs::models::{
    ComponentParams, CurrentSourceParams, NetworkParams, NetworkState, NodeParams, NodeType,
    ResistorParams,
};
use dpsimrs::simulation::Simulation;

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

    let r1 = ComponentParams::Resistor(ResistorParams {
        id: String::from("r1"),
        resistance: 1.0,
        node_1: grd,
        node_2: n1,
    });

    net_params.add_component(r1);

    let c1 = ComponentParams::CurrentSource(CurrentSourceParams {
        id: String::from("c1"),
        set_point: 5.0,
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
    println!("{}", sim.net_matrix);
    println!("{}", sim.rhs_vector);

    sim.stamp(&mut net_state);
    println!("{}", sim.net_matrix);
    println!("{}", sim.rhs_vector);

    let decomp = sim.net_matrix.lu();
    let lhs_vector = decomp.solve(&sim.rhs_vector.0).expect("msg");
    println!("{}", lhs_vector);

    assert_eq!(lhs_vector[0], -5.0);
}
