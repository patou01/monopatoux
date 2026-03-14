use pyo3::prelude::*;

pub fn hello_from_rs() -> String {
    "Hello from COMTRADE core!".to_string()
}

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok(hello_from_rs())
}

#[pymodule]
fn libcomtrade(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
