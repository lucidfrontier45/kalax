<img src="logo_wide.png" alt="Logo" width="600">

Kalax is a high-performance Rust library for Time Series Feature Extraction, designed to extract meaningful statistical and structural features from time series data.

# Origin of the Name
Kalax is a portmanteau of two Sanskrit concepts, representing the essence of time-series analysis:

- Kāla (काल): Time — The fundamental flow and dimension of data.
- Lakṣaṇa (लक्षण): Feature — The distinctive marks and characteristics of a dataset.

The terminal "x" stands for Extraction. Together, Kalax signifies the "Signs of Time," reflecting our mission to distill raw temporal data into meaningful, high-performance features using Rust.

## Features

### Core Features (Always Available)
- **Statistical Features**: Mean, median, variance, standard deviation, minimum, maximum, absolute maximum, root mean square, sum values, and length
- **High Performance**: Optimized for time series operations on `&[f64]` slices
- **Type Safety**: Strong typing throughout the library with comprehensive error handling
- **Memory Efficient**: Minimal allocations in hot paths, designed for large datasets

### Optional Polars Integration (`polars` feature)
When the `polars` feature is enabled, Kalax provides seamless integration with Polars DataFrames:

- **DataFrame Support**: Extract features from DataFrames with automatic grouping and sorting
- **Batch Processing**: Process multiple time series groups efficiently
- **Flexible Column Selection**: Automatically identify feature columns or specify custom selections
- **Result Assembly**: Create result DataFrames with properly named feature columns

## Installation

Add Kalax to your `Cargo.toml`:

### Basic Usage (Core Features Only)
```toml
[dependencies]
kalax = "0.1.0"
```

### With Polars Integration
```toml
[dependencies]
kalax = { version = "0.1.0", features = ["polars"] }
```

## Usage

### Core Feature Extraction

```rust
use kalax::features::{minimal::Mean, common::FeatureFunction};

let time_series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let mean_value = Mean::DEFAULT.apply(&time_series);
println!("Mean: {}", mean_value[0].value); // 3.0
```

### Polars DataFrame Integration (requires `polars` feature)

```rust
use kalax::polars::extract_features;
use polars::prelude::*;

// Create a DataFrame with time series data
let df = df!(
    "id" => &["A", "A", "B", "B"],
    "time" => &[1, 2, 1, 2],
    "value1" => &[1.0, 2.0, 3.0, 4.0],
    "value2" => &[5.0, 6.0, 7.0, 8.0]
).unwrap();

// Extract features grouped by 'id', sorted by 'time'
let result = extract_features(df, "id", "time")?;
// Result DataFrame has columns: id, value1__mean, value1__length, value1__variance, etc.
```

## Available Features

### Statistical Features
- **Mean**: Average value of the time series
- **Median**: Middle value when sorted
- **Variance**: Measure of spread
- **Standard Deviation**: Square root of variance
- **Minimum**: Smallest value
- **Maximum**: Largest value
- **Absolute Maximum**: Largest absolute value
- **Root Mean Square**: RMS value
- **Sum Values**: Sum of all values
- **Length**: Number of data points

### Polars Integration Features
- **Automatic Column Detection**: Identifies feature columns automatically
- **Group-wise Processing**: Process multiple time series groups
- **Sorting**: Automatically sorts time series by specified column
- **Result Assembly**: Creates structured output DataFrames
- **Error Handling**: Comprehensive error reporting

## Performance

Kalax is designed for high-performance time series analysis:

- **Memory Efficient**: Minimal allocations in feature extraction
- **Optimized Algorithms**: Efficient implementations of statistical features
- **Scalable**: Handles large datasets effectively
- **Zero-Copy Operations**: When possible, operates on data without copying

## Testing

Run the test suite:

```bash
# Test core features
cargo test

# Test with Polars integration
cargo test --features polars
```

## Development

### Building
```bash
cargo build                    # Debug build
cargo build --release         # Release build
cargo check                   # Quick compilation check
```

### Linting and Formatting
```bash
cargo clippy                  # Run linter
cargo fmt                     # Format code
```

### Feature Development
The library uses conditional compilation for optional features. Core functionality is always available, while the Polars integration is enabled via the `polars` feature flag.

## License

This project is licensed under the [LICENSE](LICENSE) file.

## Contributing

Contributions are welcome! Please ensure:
- Code follows Rust conventions and passes `cargo clippy`
- All tests pass
- Documentation is updated for new features
- Changes respect the feature flag system

## Comparison with tsfresh

Kalax provides a subset of features comparable to Python's tsfresh library, but with:
- **Better Performance**: Rust's zero-cost abstractions and efficient memory management
- **Type Safety**: Compile-time guarantees and comprehensive error handling
- **Memory Efficiency**: Minimal allocations and optimized data structures
- **Easy Integration**: Simple Rust-native API with optional Polars support