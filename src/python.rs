use pyo3::prelude::*;

use crate::{
    models::{
        CurrentSource, CurrentSourceParams, Line, LineParams, NetworkParams, NetworkState, Node,
        NodeParams, NodeType,
    },
    simulation::Simulation,
};

#[pymodule]
fn dpsimrs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<NetworkParams>()?;
    m.add_class::<LineParams>()?;
    m.add_class::<CurrentSourceParams>()?;
    m.add_class::<NodeType>()?;
    m.add_class::<NodeParams>()?;

    m.add_class::<Node>()?;
    m.add_class::<Line>()?;
    m.add_class::<CurrentSource>()?;

    m.add_class::<Simulation>()?;

    m.add_class::<NetworkState>()?;
    Ok(())
}
