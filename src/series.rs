use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum SeriesData {
    Float(Vec<f64>),
    Integer(Vec<i64>),
    String(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Series {
    name: String,
    data: SeriesData,
}

impl Series {
    pub fn new<T: Into<String>>(name: T, data: SeriesData) -> Self {
        Series {
            name: name.into(),
            data,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> usize {
        match &self.data {
            SeriesData::Float(v) => v.len(),
            SeriesData::Integer(v) => v.len(),
            SeriesData::String(v) => v.len(),
        }
    }

    pub fn get(&self, index: usize) -> Option<String> {
        match &self.data {
            SeriesData::Float(v) => v.get(index).map(|x| x.to_string()),
            SeriesData::Integer(v) => v.get(index).map(|x| x.to_string()),
            SeriesData::String(v) => v.get(index).cloned(),
        }
    }

    pub fn set(&mut self, index: usize, value: &str) -> Result<(), &'static str> {
        if index >= self.len() {
            return Err("Index out of bounds");
        }

        match &mut self.data {
            SeriesData::Float(v) => {
                v[index] = value.parse().map_err(|_| "Invalid float value")?;
            }
            SeriesData::Integer(v) => {
                v[index] = value.parse().map_err(|_| "Invalid integer value")?;
            }
            SeriesData::String(v) => {
                v[index] = value.to_string();
            }
        }
        Ok(())
    }

    pub fn push(&mut self, value: &str) -> Result<(), &'static str> {
        match &mut self.data {
            SeriesData::Float(v) => {
                let val = value.parse().map_err(|_| "Invalid float value")?;
                v.push(val);
            }
            SeriesData::Integer(v) => {
                let val = value.parse().map_err(|_| "Invalid integer value")?;
                v.push(val);
            }
            SeriesData::String(v) => {
                v.push(value.to_string());
            }
        }
        Ok(())
    }

    pub fn sum(&self) -> Option<f64> {
        match &self.data {
            SeriesData::Float(v) => Some(v.iter().sum()),
            SeriesData::Integer(v) => Some(v.iter().map(|&x| x as f64).sum()),
            SeriesData::String(_) => None,
        }
    }

    pub fn mean(&self) -> Option<f64> {
        match &self.data {
            SeriesData::Float(v) => {
                let sum: f64 = v.iter().sum();
                Some(sum / v.len() as f64)
            }
            SeriesData::Integer(v) => {
                let sum: i64 = v.iter().sum();
                Some(sum as f64 / v.len() as f64)
            }
            SeriesData::String(_) => None,
        }
    }

    pub fn head(&self, n: usize) -> Series {
        let new_data = match &self.data {
            SeriesData::Float(v) => SeriesData::Float(v.iter().take(n).cloned().collect()),
            SeriesData::Integer(v) => SeriesData::Integer(v.iter().take(n).cloned().collect()),
            SeriesData::String(v) => SeriesData::String(v.iter().take(n).cloned().collect()),
        };
        Series::new(self.name.clone(), new_data)
    }

    pub fn tail(&self, n: usize) -> Series {
        let new_data = match &self.data {
            SeriesData::Float(v) => {
                SeriesData::Float(v.iter().rev().take(n).rev().cloned().collect())
            }
            SeriesData::Integer(v) => {
                SeriesData::Integer(v.iter().rev().take(n).rev().cloned().collect())
            }
            SeriesData::String(v) => {
                SeriesData::String(v.iter().rev().take(n).rev().cloned().collect())
            }
        };
        Series::new(self.name.clone(), new_data)
    }
}
