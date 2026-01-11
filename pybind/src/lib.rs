use std::collections::HashMap;

use pyo3::prelude::*;

#[pyfunction]
pub fn extract_features(
    data: &[HashMap<String, &[f64]>],
) -> PyResult<Vec<HashMap<String, HashMap<String, f64>>>> {
    todo!()
}

/// A Python module implemented in Rust.
#[pymodule]
mod pybind {
    use pyo3::prelude::*;
}
