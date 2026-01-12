# Kalax Python Interface

Python bindings for Kalax, a high-performance time series feature extraction library written in Rust.

## Overview

Kalax provides fast, parallelized extraction of statistical features from time series data. The Python interface offers the same functionality as the Rust core with minimal overhead.

## Installation

```bash
[uv] pip install kalax
```

## Quick Start

```python
import numpy as np
from kalax import extract_features

x = np.linspace(0, 1, 100)
y = np.sin(2 * np.pi * 5 * x) + 0.5
data = [{"x": x, "y": y}]
features = extract_features(data)
```

## API

### extract_features(data)

Extracts features from multiple time series in parallel.

**Parameters:**

- `data` (List[Dict[str, np.ndarray]]): A list of dictionaries where each dictionary maps column names to 1D numpy arrays representing time series data.

**Returns:**

- List[Dict[str, Dict[str, float]]]: A list of dictionaries where each entry corresponds to an input dictionary. Feature names are mapped to dictionaries containing computed feature values.

## Available Features

| Feature              | Description                              |
| -------------------- | ---------------------------------------- |
| `mean`               | Arithmetic average of time series values |
| `median`             | Middle value when time series is sorted  |
| `variance`           | Population variance                      |
| `standard_deviation` | Square root of variance                  |
| `minimum`            | Smallest value in time series            |
| `maximum`            | Largest value in time series             |
| `absolute_maximum`   | Largest absolute value                   |
| `root_mean_square`   | RMS value                                |
| `sum_values`         | Sum of all values                        |
| `length`             | Number of data points                    |

## Example

```python
import numpy as np
from kalax import extract_features

x = np.linspace(0, 1, 100)
y = np.sin(2 * np.pi * 5 * x) + 0.5
data = [{"x": x, "y": y}]
features = extract_features(data)

print(features[0]["x"]["mean"])       # Access specific feature
print(features[0]["y"]["variance"])   # Access another feature
```

## Output Structure

```python
[
    {
        'column_name': {
            'mean': 0.5,
            'median': 0.5,
            'variance': 0.085,
            'standard_deviation': 0.292,
            'minimum': 0.0,
            'maximum': 1.0,
            'absolute_maximum': 1.0,
            'root_mean_square': 0.579,
            'sum_values': 50.0,
            'length': 100.0
        }
    }
]
```

## Requirements

- Python 3.12+
- numpy

## License

MIT
