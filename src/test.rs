use super::*;
use crate::dataframe::DataFrame;
use crate::series::{Series, SeriesData};

#[test]
fn test_series_creation() {
    let float_series = Series::new("floats", SeriesData::Float(vec![1.0, 2.0, 3.0]));
    let int_series = Series::new("ints", SeriesData::Integer(vec![1, 2, 3]));
    let string_series = Series::new(
        "strings",
        SeriesData::String(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
    );

    assert_eq!(float_series.len(), 3);
    assert_eq!(int_series.len(), 3);
    assert_eq!(string_series.len(), 3);
}

#[test]
fn test_series_operations() {
    let mut float_series = Series::new("floats", SeriesData::Float(vec![1.0, 2.0, 3.0]));

    assert_eq!(float_series.sum(), Some(6.0));
    assert_eq!(float_series.mean(), Some(2.0));

    float_series.set(1, "4.0").unwrap();
    assert_eq!(float_series.get(1).unwrap(), "4");

    float_series.push("5.0").unwrap();
    assert_eq!(float_series.len(), 4);
}

#[test]
fn test_dataframe_creation() {
    let mut df = DataFrame::new();
    df.add_series(Series::new(
        "floats",
        SeriesData::Float(vec![1.0, 2.0, 3.0]),
    ));
    df.add_series(Series::new("ints", SeriesData::Integer(vec![1, 2, 3])));

    assert_eq!(df.shape(), (3, 2));
}

#[test]
fn test_dataframe_operations() {
    let mut df = DataFrame::new();
    df.add_series(Series::new(
        "floats",
        SeriesData::Float(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    ));
    df.add_series(Series::new(
        "ints",
        SeriesData::Integer(vec![1, 2, 3, 4, 5]),
    ));

    let head_df = df.head(3);
    assert_eq!(head_df.shape(), (3, 2));

    let tail_df = df.tail(2);
    assert_eq!(tail_df.shape(), (2, 2));
}

#[test]
fn test_csv_operations() {
    use crate::io::{read_csv, write_csv};
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Create a temporary CSV file
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "floats,ints,strings").unwrap();
    writeln!(temp_file, "1.0,1,a").unwrap();
    writeln!(temp_file, "2.0,2,b").unwrap();
    writeln!(temp_file, "3.0,3,c").unwrap();

    // Read the CSV
    let df = read_csv(temp_file.path()).unwrap();
    assert_eq!(df.shape(), (3, 3));

    // Write the DataFrame to a new CSV
    let output_file = NamedTempFile::new().unwrap();
    write_csv(&df, output_file.path()).unwrap();

    // Read the new CSV and verify
    let new_df = read_csv(output_file.path()).unwrap();
    assert_eq!(new_df.shape(), (3, 3));
}
