use pyo3::prelude::*;
use pyo3::PyIterProtocol;

#[pyclass]
struct SetIterator {
    iter: std::collections::hash_set::IntoIter<usize>
}

#[pyproto]
impl PyIterProtocol for SetIterator {
    fn __iter__(slf: PyRef<Self>) -> Py<SetIterator> {
        slf.into()
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<usize> {
        slf.iter.next()
    }
}

#[pyclass]
struct HashSet {
    inner: std::collections::HashSet<usize>
}

#[pymethods]
impl HashSet {
    #[new]
    pub fn new() -> Self {
        HashSet {
            inner: std::collections::HashSet::new()
        }
    }

    pub fn insert(&mut self, value: usize) {
        self.inner.insert(value);
    }

    pub fn contains(&self, value: usize) -> bool {
        self.inner.contains(&value)
    }

    pub fn len(&mut self) -> usize {
        self.inner.len()
    }

    pub fn iter(slf: PyRef<Self>) -> PyResult<Py<SetIterator>> {
        let iter = SetIterator {
            iter: slf.inner.clone().into_iter()
        };

        Py::new(slf.py(), iter)
    }
}

#[pymodule]
pub fn hash_set(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HashSet>()?;

    Ok(())
}

