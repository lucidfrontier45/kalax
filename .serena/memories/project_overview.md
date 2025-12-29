# Kalax Project Overview

## Purpose
Kalax is a Rust library for time series feature extraction, designed to provide efficient implementations of statistical, temporal, and structural features commonly used in time series analysis. The library aims to be compatible with the tsfresh Python library's feature set.

## Architecture
- Core Rust library with minimal dependencies (currently std only)
- Planned Polars DataFrame integration for columnar operations
- Future Python bindings using PyO3 and pyo3-polars
- Focus on `&[f64]` slices for time series data (matches tsfresh convention)

## Development Roadmap
1. Implement feature extraction functions for `&[f64]` slices
2. Create Polars DataFrame wrapper for column-wise operations
3. Add Python bindings via PyO3

## Tech Stack
- **Language**: Rust (2024 edition)
- **Build System**: Cargo
- **Planned Dependencies**: Polars, PyO3
- **Development Tools**: rustfmt, Clippy, rust-analyzer
- **IDE**: VSCode with Rust extensions