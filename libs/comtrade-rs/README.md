# COMTRADE-RS

A high-performance COMTRADE (IEEE C37.111) file reader written in Rust with Python bindings.

## Project Structure

- `rs/`: Rust core library. It uses `PyO3` to provide a Python module.
- `python/`: Pure Python wrapper that includes the compiled Rust library as a data dependency.

## Building

The project is integrated into the monorepo and built using Bazel.

To build the Rust shared library:
```bash
bazel build //libs/comtrade-rs/rs:comtrade_so
```

To build the Python library (which depends on the Rust library):
```bash
bazel build //libs/comtrade-rs/python:comtrade_py
```

To run the tests:
```bash
bazel test //libs/comtrade-rs/python:test_hello
```

## How it works

The library is designed to be highly performant by using Rust for the heavy lifting (parsing binary records) and exposing a clean API to Python.

### Rust Layer (`rs/`)
The Rust core is compiled into a shared library (`.so` on Linux, `.pyd` on Windows). It uses the Stable ABI (`abi3`) to ensure compatibility across different Python versions without needing to recompile for each minor version.

### Python Layer (`python/`)
The Python package `comtrade` imports the functions and classes directly from the compiled Rust module. It is intended to be used as a regular Python package.

### Expected Usage
Once integrated into your Python environment, you can use it like this:

```python
from comtrade import hello

# Initial hello function to verify integration
print(hello())
```

In the future, the API will support loading `.CFG` and `.DAT` files:
```python
import comtrade

# Load a COMTRADE record
record = comtrade.load("path/to/file.cfg")

# Access metadata
print(record.station_name)

# Access signal data as numpy-compatible arrays
for channel in record.analog_channels:
    print(channel.label, channel.data)
```
