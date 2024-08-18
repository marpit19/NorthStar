# NorthStar

NorthStar is a data processing library implemented in Rust with Python bindings.

## Installation

```
pip install northstar
```

## Usage

Here's a basic example of how to use NorthStar:

```python
import northstar

# Create a Series
s = northstar.PySeries("example", [1, 2, 3, 4, 5])
print(f"Series sum: {s.sum()}")
print(f"Series mean: {s.mean()}")

# Create a DataFrame
df = northstar.PyDataFrame()
df.add_series(s)

# CSV operations
df.to_csv("example.csv")
df2 = northstar.read_csv("example.csv")
print(f"DataFrame from CSV: {df2}")
```

## Development

To set up the development environment:

1. Clone the repository
2. Install maturin: `pip install maturin`
3. Build and install the package: `maturin develop`

## Running Tests

Run the test script:

```
python test_northstar.py
```

## License

This project is licensed under the MIT License.