use crate::series::Series;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DataFrame {
    data: HashMap<String, Series>,
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
