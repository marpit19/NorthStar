use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod dataframe;
mod io;
mod series;

#[pymodule]
fn northstar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<series::PySeries>()?;
    m.add_class::<dataframe::PyDataFrame>()?;
    m.add_function(wrap_pyfunction!(io::py_read_csv, m)?)?;
    m.add_function(wrap_pyfunction!(io::py_write_csv, m)?)?;
    Ok(())
}
