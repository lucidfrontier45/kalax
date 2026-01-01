# Kalax Code Style and Conventions

## Rust Edition & Compiler
- Use Rust 2024 edition
- Enable all stable features
- Follow Rust 2024 idioms and best practices

## Naming Conventions

### Functions & Methods
- Use `snake_case` for function names
- Start with verb when possible (e.g., `extract_features`, `calculate_mean`)
- Be descriptive but concise

### Variables
- Use `snake_case` for all variables
- Use descriptive names (e.g., `time_series`, `feature_values`)
- Avoid single-letter names except for simple loops

### Types & Structs
- Use `PascalCase` for type names
- Keep names concise but descriptive
- Use full words over abbreviations

### Constants
- Use `SCREAMING_SNAKE_CASE`
- Place in appropriate modules

## Imports & Modules

### Import Style (VSCode enforced)
- Use plain import prefix style
- Enforce import granularity grouping
- Group and merge imports by crate
- Enable import grouping
- **DO NOT use glob imports (`use crate::*`)** - Always use explicit imports
- Exception: Prelude modules (e.g., `rayon::prelude::*`) are allowed for established patterns

### Import Organization Example
```rust
// Standard library imports
use std::collections::HashMap;

// External crate imports
use polars::prelude::{DataFrame, Series};

// Local crate imports
use crate::features::{mean, extract_features};

// Super imports
use super::common::{validate_input, Error};
```

## Error Handling
- Use `Result<T, E>` for fallible operations
- Define custom error types for library operations
- Use `?` operator for error propagation
- Provide meaningful error messages

## Documentation
- Document all public APIs with `///` comments
- Use markdown in doc comments
- Include examples for complex functions
- Document parameters, return values, and panics
- Document all public APIs with `///` comments
- Use markdown in doc comments
- Include examples for complex functions
- Document parameters, return values, and panics

## Testing
- Write unit tests for all public functions
- Use descriptive test names (e.g., `test_calculate_mean_with_empty_slice`)
- Test edge cases (empty inputs, boundary values)
- Use `assert_eq!` for value comparisons
- Test error conditions appropriately

## Type Safety
- Use strong typing throughout
- Prefer concrete types over generics when possible
- Use `&[f64]` for time series data (matches tsfresh convention)
- Avoid `as` casts; use explicit conversions

## Performance Considerations
- Optimize for time series operations on `&[f64]`
- Minimize allocations in hot paths
- Use iterators where possible
- Consider cache efficiency for numerical computations
- use `ndarray::ArrayView1` if necessary.

## Code Organization

### File Structure
```
src/
  lib.rs              # Main library file
  features/           # Feature extraction modules
    statistical.rs    # Statistical features (mean, std, etc.)
    temporal.rs       # Time-based features
    structural.rs     # Structural features
  dataframe.rs        # Polars DataFrame integration
  python.rs           # Python bindings (future)
```

### Module Organization
- Group related functionality into modules
- Use clear module boundaries
- Re-export important items in `lib.rs`
- **NEVER use `mod.rs` files** - Prefer directory-style modules with `directory_name.rs` files instead

## Formatting & Linting
- Format on save enabled in VSCode (rustfmt defaults)
- Treat Clippy warnings as errors
- Use `cargo clippy --fix` for auto-fixes when safe