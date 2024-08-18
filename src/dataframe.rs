use crate::series::{PySeries, Series};
use pyo3::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DataFrame {
    data: HashMap<String, Series>,
}

#[pyclass]
pub struct PyDataFrame {
    pub(crate) inner: DataFrame,
}

impl DataFrame {
    pub fn new() -> Self {
        DataFrame {
            data: HashMap::new(),
        }
    }

    pub fn add_series(&mut self, series: Series) {
        self.data.insert(series.name().to_string(), series);
    }

    pub fn get_series(&self, name: &str) -> Option<&Series> {
        self.data.get(name)
    }

    pub fn get_series_mut(&mut self, name: &str) -> Option<&mut Series> {
        self.data.get_mut(name)
    }

    pub fn shape(&self) -> (usize, usize) {
        let rows = self.data.values().next().map_or(0, |s| s.len());
        let cols = self.data.len();
        (rows, cols)
    }

    pub fn head(&self, n: usize) -> DataFrame {
        let mut new_df = DataFrame::new();
        for (_, series) in &self.data {
            new_df.add_series(series.head(n));
        }
        new_df
    }

    pub fn tail(&self, n: usize) -> DataFrame {
        let mut new_df = DataFrame::new();
        for (_, series) in &self.data {
            new_df.add_series(series.tail(n));
        }
        new_df
    }

    pub fn series_names(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }
}

#[pymethods]
impl PyDataFrame {
    #[new]
    pub fn new() -> Self {
        PyDataFrame {
            inner: DataFrame::new(),
        }
    }

    pub fn add_series(&mut self, series: &PySeries) -> PyResult<()> {
        self.inner.add_series(series.inner.clone());
        Ok(())
    }

    pub fn get_series(&self, name: &str) -> Option<PySeries> {
        self.inner
            .get_series(name)
            .map(|s| PySeries { inner: s.clone() })
    }

    pub fn shape(&self) -> (usize, usize) {
        self.inner.shape()
    }

    pub fn head(&self, n: usize) -> PyResult<PyDataFrame> {
        Ok(PyDataFrame {
            inner: self.inner.head(n),
        })
    }

    pub fn tail(&self, n: usize) -> PyResult<PyDataFrame> {
        Ok(PyDataFrame {
            inner: self.inner.tail(n),
        })
    }

    fn __repr__(&self) -> String {
        format!("PyDataFrame with shape {:?}", self.shape())
    }

    fn __getitem__(&self, key: &str) -> PyResult<PySeries> {
        self.get_series(key).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("No series named '{}'", key))
        })
    }
}
