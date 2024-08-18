import northstar as ns
import time
import numpy as np


def main():
    print("🚀 Welcome to the NorthStar Demo! 🌟")
    print("Let's crunch some data faster than you can say 'Rust is awesome'! 🦀\n")

    # Create a DataFrame
    df = ns.PyDataFrame()
    df.add_series(ns.PySeries("ages", [25, 30, 22, 28, 33, 21, 27]))
    df.add_series(ns.PySeries(
        "names", ["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace"]))
    df.add_series(ns.PySeries(
        "scores", [88.5, 92.0, 79.5, 95.5, 87.0, 91.5, 84.0]))

    print("📊 Here's our cool DataFrame:")
    print(df)
    print(f"Shape: {df.shape()}\n")

    # Performance comparison
    print("\n⚡ Now, let's see how fast NorthStar is!")

    data = list(range(1_000_000))

    # NorthStar
    start_time = time.time()
    ns_series = ns.PySeries("big_data", data)
    ns_sum = ns_series.sum()
    ns_time = time.time() - start_time

    print(f"NorthStar sum: {ns_sum}")
    print(f"NorthStar time: {ns_time:.4f} seconds")

    # Pure Python
    start_time = time.time()
    py_sum = sum(data)
    py_time = time.time() - start_time

    print(f"Python sum: {py_sum}")
    print(f"Python time: {py_time:.4f} seconds")

    # NumPy
    np_data = np.array(data)
    start_time = time.time()
    np_sum = np.sum(np_data)
    np_time = time.time() - start_time

    print(f"NumPy sum: {np_sum}")
    print(f"NumPy time: {np_time:.4f} seconds")

    # Calculate speedups
    ns_vs_py_speedup = py_time / ns_time
    ns_vs_np_speedup = np_time / ns_time

    print(
        f"\n🏎️ NorthStar is {ns_vs_py_speedup:.2f}x faster than pure Python!")
    print(f"🚀 NorthStar is {ns_vs_np_speedup:.2f}x faster than NumPy! 🤯")

    print("\n🎉 That's all, folks! NorthStar: Making data science cooler and faster! 😎")


if __name__ == "__main__":
    main()
