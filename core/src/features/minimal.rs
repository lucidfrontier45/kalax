//! Minimal feature extraction module.
//!
//! Provides basic statistical and time series features.
//! Contains both functional and OOP APIs.

mod functional;
mod oop;

// Re-export common items for convenience
pub use functional::{
    absolute_maximum, length, maximum, mean, median, minimum, root_mean_square, standard_deviation,
    sum_values, variance,
};
pub use oop::{
    AbsoluteMaximum, Length, Maximum, Mean, Median, MinimalFeatureSet, Minimum, RootMeanSquare,
    StandardDeviation, SumValues, Variance,
};
