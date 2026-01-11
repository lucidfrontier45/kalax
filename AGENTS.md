# Kalax Development Guide

This guide provides essential information for AI agents working on the Kalax codebase. Kalax is a Rust library for time series feature extraction with both functional and object-oriented APIs.

## Project Overview

- **Language**: Rust (2024 edition)
- **Purpose**: Time series feature extraction library
- **Architecture**: Core Rust library with dual API design (functional + OOP), Rayon-based parallel batch processing, planned Python bindings
- **Dependencies**: Minimal (Rayon for parallelization, serde/serdeio for testing)

## Build Commands

### Build
```bash
cargo build                    # Debug build
cargo build --release         # Release build
cargo check                   # Check for compilation errors without building
```

### Test
```bash
cargo test                    # Run all tests
cargo test <test_name>        # Run specific test (e.g., cargo test it_works)
cargo test --lib              # Run only library tests
cargo test --doc              # Run documentation tests
cargo test -- --nocapture     # Show println! output in tests
```

### Lint & Format
```bash
cargo clippy                  # Run Clippy linter
cargo clippy --fix            # Auto-fix Clippy suggestions
cargo fmt                     # Format code with rustfmt
cargo fmt --check             # Check formatting without modifying files
```

### Documentation
```bash
cargo doc                     # Generate documentation
cargo doc --open              # Generate and open documentation in browser
```

## Code Style Guidelines

### Rust Edition & Compiler
- Use Rust 2024 edition
- Enable all stable features
- Follow Rust 2024 idioms and best practices

### Formatting (rustfmt)
- Use default rustfmt configuration
- Format on save is enabled in VSCode settings
- Maximum line width follows rustfmt defaults

### Linting (Clippy)
- Treat Clippy warnings as errors
- Use `cargo clippy` for linting
- Auto-fix with `cargo clippy --fix` when safe

### Naming Conventions

#### Functions & Methods
- Use `snake_case` for function names
- Start with verb when possible (e.g., `extract_features`, `calculate_mean`)
- Be descriptive but concise

#### Variables
- Use `snake_case` for all variables
- Use descriptive names (e.g., `time_series`, `feature_values`)
- Avoid single-letter names except for simple loops

#### Types & Structs
- Use `PascalCase` for type names
- Keep names concise but descriptive
- Use full words over abbreviations

#### Constants
- Use `SCREAMING_SNAKE_CASE`
- Place in appropriate modules

### Imports & Modules

#### Import Style (from VSCode settings)
- Use plain import prefix style
- Enforce import granularity grouping
- Group and merge imports by crate
- Enable import grouping
- **DO NOT use glob imports (`use crate::*`)** - Always use explicit imports
- Exception: Prelude modules (e.g., `rayon::prelude::*`) are allowed for established patterns

#### Import Organization
```rust
// Standard library imports
use std::collections::HashMap;

// External crate imports
// use polars::prelude::{DataFrame, Series};  // Polars integration removed

// Local crate imports
use crate::features::{mean, extract_features};

// Super imports
use super::common::{validate_input, Error};
```

### Error Handling
- Use `Result<T, E>` for fallible operations
- Define custom error types for library operations
- Use `?` operator for error propagation
- Provide meaningful error messages

### Documentation
- Document all public APIs with `///` comments
- Use markdown in doc comments
- Include examples for complex functions
- Document parameters, return values, and panics

### Testing
- Write unit tests for all public functions
- Use descriptive test names (e.g., `test_calculate_mean_with_empty_slice`)
- Test edge cases (empty inputs, boundary values)
- Use `assert_float_eq!` for floating-point comparisons (exported from lib.rs as macro)
- Test error conditions appropriately

### Type Safety
- Use strong typing throughout
- Prefer concrete types over generics when possible
- Use `&[f64]` for time series data (matches tsfresh convention)
- Avoid `as` casts; use explicit conversions

#### Performance Considerations
- Use Rayon for parallel processing across multiple time series
- Operate on slices `&[f64]` for minimal allocations
- Use iterators where possible
- Consider cache efficiency for numerical computations

### Code Organization

#### File Structure
```
src/
  lib.rs              # Main library file, re-exports
  extractor.rs        # Batch feature extraction API
  features/           # Feature extraction modules
    common.rs         # Traits and common structures
    minimal.rs        # Minimal feature set re-exports
    minimal/          # Minimal feature implementations
      functional.rs  # Functional API (10 features)
      oop.rs        # Object-Oriented API (10 structs + MinimalFeatureSet)
  test_utils.rs       # Testing utilities (assert_float_eq! macro)
```

#### Module Organization
- Group related functionality into modules
- Use clear module boundaries
- Re-export important items in `lib.rs`
- **NEVER use `mod.rs` files** - Prefer directory-style modules with `directory_name.rs` files instead

### Implemented Features

#### Statistical Features (Minimal Feature Set)
The library implements 10 core statistical features, available through both APIs:

1. **Mean** (`mean`, `Mean`) - Arithmetic average of time series values
2. **Median** (`median`, `Median`) - Middle value when time series is sorted
3. **Variance** (`variance`, `Variance`) - Measure of spread using population variance
4. **Standard Deviation** (`standard_deviation`, `StandardDeviation`) - Square root of variance
5. **Minimum** (`minimum`, `Minimum`) - Smallest value in time series
6. **Maximum** (`maximum`, `Maximum`) - Largest value in time series
7. **Absolute Maximum** (`absolute_maximum`, `AbsoluteMaximum`) - Largest absolute value
8. **Root Mean Square** (`root_mean_square`, `RootMeanSquare`) - RMS value
9. **Sum Values** (`sum_values`, `SumValues`) - Sum of all time series values
10. **Length** (`length`, `Length`) - Number of data points in time series

All features operate on `&[f64]` slices for memory efficiency and return `f64` values (functional API) or `FeatureFunctionReturn` structs (OOP API).

### Feature Extraction Patterns

#### Function Signatures
```rust
/// Calculate the mean of a time series
pub fn mean(series: &[f64]) -> f64 {
    // Implementation
}

/// Extract multiple features from multiple time series in parallel
pub fn extract_features(data: &[HashMap<String, &[f64]>]) -> Vec<HashMap<String, HashMap<String, f64>>> {
    // Implementation
}
```

#### Return Types
- Use `f64` for scalar features (functional API)
- Use `Vec<FeatureFunctionReturn>` for structured results (OOP API)
- Use `HashMap<String, f64>` for named feature collections
- Use `Vec<HashMap<String, HashMap<String, f64>>>` for batch processing results
- Return `Result<T, E>` for operations that can fail

### Integration Patterns

#### Batch Processing API
- Use `extract_features()` function for parallel processing of multiple time series
- Input format: `&[HashMap<String, &[f64]>]` - vector of time series collections
- Output format: `Vec<HashMap<String, HashMap<String, f64>>>` - batch results with named features
- Rayon-based parallelization across multiple time series for optimal performance

#### Usage Examples

##### Batch Feature Extraction
```rust
use std::collections::HashMap;
use kalax::extract_features;

// Prepare data as vector of HashMaps (column name -> time series values)
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

// results[0]["sensor1"] contains features for sensor1 from first series
// results[1]["sensor2"] contains features for sensor2 from second series
```

#### Future Integration Options
- Python bindings planned via PyO3 for ML ecosystem integration
- Data library integration (Polars, Arrow) may be reconsidered based on user demand
- Current focus on core `&[f64]` slice operations for maximum performance

#### Python Bindings (Future)
- Use PyO3 for Python integration
- Follow PyO3 best practices
- Provide Pythonic APIs
- Handle type conversions carefully

### Development Workflow

#### Before Committing
1. Run `cargo fmt` to format code
2. Run `cargo clippy` to check for issues
3. Run `cargo test` to ensure tests pass
4. Run `cargo doc` to check documentation

#### Testing Strategy
- Unit tests for individual functions
- Integration tests for batch operations on Vec<HashMap<String, f64>>
- Property-based tests for mathematical correctness
- Benchmark tests for performance-critical code

#### Version Control
- Use descriptive commit messages
- Follow conventional commit format when possible
- Keep commits focused and atomic
- Update documentation with code changes

### Tool Configuration

#### VSCode Settings (from .vscode/settings.json)
- Format on save enabled for Rust files
- Use Clippy for checking
- Import grouping and granularity enforced
- Test explorer enabled

#### Serena Code Analysis Tools
Serena provides intelligent code analysis and exploration tools for efficient development:

- **Symbol Analysis**: Use `serena_get_symbols_overview` for file structure, `serena_find_symbol` for locating functions/classes, `serena_find_referencing_symbols` for dependencies
- **Code Search**: Use `serena_search_for_pattern` for flexible pattern matching across files
- **Symbol Editing**: Use `serena_replace_symbol_body`, `serena_insert_after_symbol`, `serena_insert_before_symbol` for precise code modifications
- **Project Navigation**: Use `serena_list_dir` for directory exploration, `serena_find_file` for file location
- **Memory System**: Use `serena_write_memory`/`serena_read_memory` for storing and retrieving project knowledge

Prioritize Serena tools over basic file operations for efficient code understanding and modification.

#### Future CI/CD
- GitHub Actions for automated testing
- Test on multiple Rust versions
- Check formatting and linting
- Run benchmarks on PRs

### Security Considerations
- Validate input data ranges
- Handle floating-point edge cases (NaN, infinity)
- Avoid unsafe code unless absolutely necessary
- Document any unsafe code blocks thoroughly

### Performance Benchmarks
- Benchmark feature extraction functions
- Compare against reference implementations (tsfresh)
- Monitor memory usage for large datasets
- Profile hot paths regularly

---

This guide should be updated as the project evolves. If you encounter inconsistencies between this guide and the codebase, update the guide to match current practices.