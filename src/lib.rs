mod handlers;
mod server;

use pyo3::prelude::*;

use crate::server::{Meteorite};

#[pymodule]
fn meteorite(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Meteorite>().expect("Could not add class");
    Ok(())
}
