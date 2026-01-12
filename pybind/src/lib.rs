use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule(name = "kalax")]
mod kalax_pybind {
    use std::collections::HashMap;

    use numpy::PyReadonlyArray1;
    use pyo3::prelude::*;

    #[pyfunction]
    pub fn extract_features<'py>(
        data: Vec<HashMap<String, PyReadonlyArray1<'py, f64>>>,
    ) -> PyResult<Vec<HashMap<String, HashMap<String, f64>>>> {
        let x: PyResult<Vec<HashMap<String, &[f64]>>> = data
            .iter()
            .map(|item| {
                item.iter()
                    .map(|(k, v)| Ok((k.clone(), v.as_slice()?)))
                    .collect()
            })
            .collect();
        let features = kalax::extract_features(&x?);
        Ok(features)
    }
}
