use components::{Network, NodeType, NodeParams, ComponentParams, ResistorParams, CurrentSourceParams};
use simulation::Simulation;

mod math;
mod components;
mod simulation;

fn main() {
    
    let mut net = Network::new();

    let grd = NodeParams{
        id: "GRD".to_string(),
        node_type: NodeType::Ground,
    };

    let n1 = NodeParams{
        id: "N1".to_string(),
        node_type: NodeType::Network,
    };

    net.add_node(&grd);
    net.add_node(&n1);

    let r1 = ComponentParams::Resistor(ResistorParams{ 
        id: String::from("r1"), 
        resistance: 1.0,
        node_1: &grd,
        node_2: &n1,
    });

    net.add_component(&r1);

    let c1 = ComponentParams::CurrentSource(CurrentSourceParams {
        id: String::from("c1"),
        set_point: 5.0,
        node_1: &n1,
        node_2: &grd,
    });

    net.add_component(&c1);
    println!("{:?}", net);

    let mut sim = simulation::Simulation::new(&mut net);

    sim.generate_sim_objects(&mut net);
    println!("Nodes: \n{:?}\n", net.nodes);
    println!("Components: \n{:?}\n", net.comps);

    println!("{}", sim.net_matrix);
    println!("{}", sim.rhs_vector);

    sim.stamp(&mut net);
    println!("{}", sim.net_matrix);
    println!("{}", sim.rhs_vector);
    //components::stamp_resistor(&res1, &mut network_matrix);
    //components::stamp_resistor(&res2, &mut network_matrix);
    //components::stamp_current_source(&csrc, &mut rhs_vector);

    //println!("{}", network_matrix);
    //println!("{}", rhs_vector);

    //let decomp = network_matrix.lu();
    //let lhs_vector = decomp.solve(&rhs_vector).expect("msg");

    //println!("{}", lhs_vector);

}
