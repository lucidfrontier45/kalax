# Kalax Project Overview

## Purpose
Kalax is a Rust library for time series feature extraction with both functional and object-oriented APIs. It provides efficient implementations of statistical, temporal, and structural features commonly used in time series analysis. The library aims to be compatible with the tsfresh Python library's feature set.

## Architecture
- Core Rust library with dual API design (functional + OOP)
- Rayon-based parallel batch processing for optimal performance
- **Polars integration was removed** to focus on core functionality
- Future Python bindings using PyO3
- Supports both `&[f64]` slices and `Vec<HashMap<String, &[f64]>]` for batch processing
- Functional and object-oriented APIs for flexible usage

## Development Roadmap
1. ✅ Implement core feature extraction functions for `&[f64]` slices
2. ✅ Add batch processing support for `Vec<HashMap<String, f64>>`
3. ✅ Implement both functional and object-oriented APIs
4. Add Python bindings via PyO3
5. Consider Polars integration based on user demand

## Tech Stack
- **Language**: Rust (2024 edition)
- **Build System**: Cargo
- **Dependencies**: Minimal (Rayon for parallelization, serde/serdeio for testing)
- **Development Tools**: rustfmt, Clippy, rust-analyzer
- **IDE**: VSCode with Rust extensions
- **Testing**: Built-in Rust testing framework with custom float comparison utilities