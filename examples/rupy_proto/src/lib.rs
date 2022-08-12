pub mod app;

use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "rupy")]
fn rupy_proto(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<app::User>()?;
    m.add_class::<app::Tweet>()?;
    Ok(())
}
