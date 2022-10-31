import dpsimrs

net_params = dpsimrs.NetworkParams()

grd = net_params.add_node(dpsimrs.NodeParams('GRD', dpsimrs.NodeType.Ground))
n1 = net_params.add_node(dpsimrs.NodeParams('N1', dpsimrs.NodeType.Network))

r1 = dpsimrs.ResistorParams('r1', 1.0, grd, n1)
net_params.add_component(r1)

c1 = dpsimrs.CurrentSourceParams('c1', 5.0, grd, n1)
net_params.add_component(c1)
