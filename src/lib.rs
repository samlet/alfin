use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod common;
mod hash_set;

use common::*;
use hash_set::*;

#[pymodule]
fn alfin(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(common))?;
    m.add_wrapped(wrap_pymodule!(hash_set))?;

    // Inserting to sys.modules allows importing submodules nicely from Python
    // e.g. from alfin.common import Point

    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("alfin.common", m.getattr("common")?)?;
    sys_modules.set_item("alfin.hash_set", m.getattr("hash_set")?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
