use pyo3::prelude::*;
mod keep_cool;
mod reader;
mod types;
#[cfg(test)]
mod tests;

#[pymodule]
fn linkapy(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(keep_cool::parse_cools, m)?)?;
    Ok(())
}