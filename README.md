<img src="logo_wide.png" alt="Logo" width="600">

Kalax is a high-performance Rust library for Time Series Feature Extraction, designed to extract meaningful statistical and structural features from time series data.

# Origin of the Name
Kalax is a portmanteau of two Sanskrit concepts, representing the essence of time-series analysis:

- Kāla (काल): Time — The fundamental flow and dimension of data.
- Lakṣaṇa (लक्षण): Feature — The distinctive marks and characteristics of a dataset.

The terminal "x" stands for Extraction. Together, Kalax signifies the "Signs of Time," reflecting our mission to distill raw temporal data into meaningful, high-performance features using Rust.

## Features

- **Statistical Features**: Mean, median, variance, standard deviation, minimum, maximum, absolute maximum, root mean square, sum values, and length
- **Dual API Design**: Both functional and object-oriented APIs for flexibility
- **High Performance**: Optimized for time series operations on `&[f64]` slices with parallel processing
- **Type Safety**: Strong typing throughout the library with comprehensive error handling
- **Memory Efficient**: Minimal allocations in hot paths, designed for large datasets
- **Batch Processing**: Extract features from multiple time series efficiently using parallel execution

## Installation

Add Kalax to your `Cargo.toml`:

```toml
[dependencies]
kalax = "0.1.0"
```

## Usage

Kalax provides two API styles: functional and object-oriented.

### Functional API

Simple function calls for individual features:

```rust
use kalax::features::minimal::{mean, variance, standard_deviation};

let time_series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let mean_value = mean(&time_series);
let variance_value = variance(&time_series);
let std_dev = standard_deviation(&time_series);

println!("Mean: {}", mean_value);           // 3.0
println!("Variance: {}", variance_value);    // 2.0
println!("Std Dev: {}", std_dev);           // ~1.414
```

### Object-Oriented API

Use the `FeatureFunction` trait for more structured code:

```rust
use kalax::features::{minimal::Mean, common::FeatureFunction};

let time_series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let result = Mean::DEFAULT.apply(&time_series);
println!("{}: {}", result[0].name, result[0].value); // mean: 3.0
```

### Extract All Minimal Features

Use `MinimalFeatureSet` to extract all supported features at once:

```rust
use kalax::features::{minimal::MinimalFeatureSet, common::FeatureFunction};

let time_series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let features = MinimalFeatureSet::new().apply(&time_series);

for feature in features {
    println!("{}: {}", feature.name, feature.value);
}
// Output: absolute_maximum, mean, median, variance, standard_deviation,
//         length, maximum, minimum, root_mean_square, sum_values
```

### Batch Processing

Process multiple time series efficiently using the extractor:

```rust
use std::collections::HashMap;
use kalax::extract_features;

// Prepare data as a vector of HashMaps (column name -> time series values)
let data = vec![
    HashMap::from([
        ("sensor1".to_string(), vec![1.0, 2.0, 3.0]),
        ("sensor2".to_string(), vec![4.0, 5.0, 6.0]),
    ]),
    HashMap::from([
        ("sensor1".to_string(), vec![7.0, 8.0, 9.0]),
        ("sensor2".to_string(), vec![10.0, 11.0, 12.0]),
    ]),
];

// Extract features in parallel
let results = extract_features(&data);

// results[0]["sensor1"] contains features for sensor1 from the first series
// results[1]["sensor2"] contains features for sensor2 from the second series
```

## Available Features

All features are available through both the functional and OOP APIs.

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

### API Styles

#### Functional API
Import and call functions directly:
```rust
use kalax::features::minimal::{mean, median, variance};
let m = mean(&series);
```

#### OOP API
Use feature structs and the `FeatureFunction` trait:
```rust
use kalax::features::{minimal::Mean, common::FeatureFunction};
let result = Mean::DEFAULT.apply(&series);
```

## Performance

Kalax is designed for high-performance time series analysis:

- **Parallel Processing**: Uses Rayon for parallel feature extraction across multiple time series
- **Memory Efficient**: Minimal allocations in feature extraction
- **Optimized Algorithms**: Efficient implementations of statistical features
- **Zero-Copy Operations**: Operates on `&[f64]` slices without copying data
- **Scalable**: Handles large datasets effectively with batch processing

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_minimal_extractor

# Show test output
cargo test -- --nocapture
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
cargo clippy --fix            # Auto-fix linter warnings
```

### Documentation
```bash
cargo doc                     # Generate documentation
cargo doc --open              # Generate and open in browser
```

### API Design Philosophy
Kalax provides two API styles to accommodate different use cases:

- **Functional API**: Best for simple, one-off feature extraction where you need direct access to specific features
- **OOP API**: Best when you need consistent interfaces, want to chain features, or need to work with feature collections

Both APIs provide identical performance; the choice is primarily about code organization and developer preference.

## License

This project is licensed under the [LICENSE](LICENSE) file.

## Contributing

Contributions are welcome! Please ensure:
- Code follows Rust conventions and passes `cargo clippy`
- All tests pass
- Documentation is updated for new features
- New features include both functional and OOP implementations

## Comparison with tsfresh

Kalax provides a subset of features comparable to Python's tsfresh library, with focus on core statistical features. Key advantages:
- **Better Performance**: Rust's zero-cost abstractions, efficient memory management, and parallel processing
- **Type Safety**: Compile-time guarantees and comprehensive error handling
- **Memory Efficiency**: Minimal allocations and optimized data structures
- **Dual API**: Both functional and object-oriented APIs for flexibility
- **Validation**: Features tested against tsfresh reference implementation for correctness