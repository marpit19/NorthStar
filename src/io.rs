use crate::dataframe::{DataFrame, PyDataFrame};
use crate::series::{Series, SeriesData};
use csv::{Reader, Writer};
use pyo3::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<DataFrame, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let headers = rdr.headers()?.clone();

    let mut df = DataFrame::new();

    for header in headers.iter() {
        df.add_series(Series::new(
            header.to_string(),
            SeriesData::String(Vec::new()),
        ));
    }

    for result in rdr.records() {
        let record = result?;
        for (i, field) in record.iter().enumerate() {
            if let Some(series) = df.get_series_mut(&headers[i]) {
                series.push(field)?;
            }
        }
    }

    Ok(df)
}

pub fn write_csv<P: AsRef<Path>>(df: &DataFrame, path: P) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;

    // Write headers
    let headers: Vec<String> = df.series_names().map(|s| s.to_string()).collect();
    wtr.write_record(&headers)?;

    // Write data
    let rows = df.shape().0;
    for i in 0..rows {
        let row: Vec<String> = headers
            .iter()
            .filter_map(|h| df.get_series(h))
            .map(|s| s.get(i).map(|v| format!("{:?}", v)).unwrap_or_default())
            .collect();
        wtr.write_record(&row)?;
    }

    wtr.flush()?;
    Ok(())
}

#[pyfunction]
pub fn py_read_csv(path: &str) -> PyResult<PyDataFrame> {
    let df =
        read_csv(path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
    Ok(PyDataFrame { inner: df })
}

#[pyfunction]
pub fn py_write_csv(df: &PyDataFrame, path: &str) -> PyResult<()> {
    write_csv(&df.inner, path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
}
