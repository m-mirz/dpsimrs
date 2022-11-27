import dpsimrs

net_params = dpsimrs.NetworkParams()

grd = net_params.add_node(dpsimrs.NodeParams('GRD', dpsimrs.NodeType.Ground))
n1 = net_params.add_node(dpsimrs.NodeParams('N1', dpsimrs.NodeType.Network))

r1 = dpsimrs.LineParams('r1', 1.0, 0.0, grd, n1)
net_params.add_component(r1)

c1 = dpsimrs.CurrentSourceParams('c1', complex(5.0, 0.0), n1, grd)
net_params.add_component(c1)

net_state = dpsimrs.NetworkState()
net_state.generate_sim_objects(net_params)

sim = dpsimrs.Simulation(net_state)
sim.stamp(net_state)

lhs_vector = sim.solve()
print(lhs_vector)
