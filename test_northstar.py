import northstar

# Test Series creation
s = northstar.PySeries("test", [1, 2, 3, 4, 5])
print(f"Series: {s}")
print(f"Sum: {s.sum()}")
print(f"Mean: {s.mean()}")

# Test DataFrame creation
df = northstar.PyDataFrame()
df.add_series(s)
print(f"DataFrame: {df}")

# Test CSV operations
df.to_csv("./assets/test.csv")
df2 = northstar.PyDataFrame.read_csv("./assets/test.csv")
print(f"DataFrame from CSV: {df2}")

valorant_df = northstar.PyDataFrame.read_csv("./assets/ValorantStats.csv")
print(f"DataFrame from CSV: {valorant_df}")
