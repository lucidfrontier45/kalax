//! OOP API for minimal feature extraction.

use crate::features::common::{FeatureFunction, FeatureFunctionReturn};

use super::functional::{
    absolute_maximum, length, maximum, mean, median, minimum, root_mean_square, standard_deviation,
    sum_values, variance,
};

/// Feature function that calculates the absolute maximum value.
#[derive(Default)]
pub struct AbsoluteMaximum;

impl AbsoluteMaximum {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for AbsoluteMaximum {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "absolute_maximum".to_string(),
            value: absolute_maximum(series),
        }]
    }
}

/// Feature function that calculates the arithmetic mean.
#[derive(Default)]
pub struct Mean;

impl Mean {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for Mean {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "mean".to_string(),
            value: mean(series),
        }]
    }
}

/// Feature function that calculates the median value.
#[derive(Default)]
pub struct Median;

impl Median {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for Median {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "median".to_string(),
            value: median(series),
        }]
    }
}

/// Feature function that calculates the variance.
#[derive(Default)]
pub struct Variance;

impl Variance {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for Variance {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "variance".to_string(),
            value: variance(series),
        }]
    }
}

/// Feature function that calculates the standard deviation.
#[derive(Default)]
pub struct StandardDeviation;

impl StandardDeviation {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for StandardDeviation {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "standard_deviation".to_string(),
            value: standard_deviation(series),
        }]
    }
}

/// Feature function that returns the length of the time series.
#[derive(Default)]
pub struct Length;

impl Length {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for Length {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "length".to_string(),
            value: length(series) as f64,
        }]
    }
}

/// Feature function that calculates the maximum value.
#[derive(Default)]
pub struct Maximum;

impl Maximum {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for Maximum {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "maximum".to_string(),
            value: maximum(series),
        }]
    }
}

/// Feature function that calculates the minimum value.
#[derive(Default)]
pub struct Minimum;

impl Minimum {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for Minimum {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "minimum".to_string(),
            value: minimum(series),
        }]
    }
}

/// Feature function that calculates the root mean square.
#[derive(Default)]
pub struct RootMeanSquare;

impl RootMeanSquare {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for RootMeanSquare {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "root_mean_square".to_string(),
            value: root_mean_square(series),
        }]
    }
}

/// Feature function that calculates the sum of values.
#[derive(Default)]
pub struct SumValues;

impl SumValues {
    pub const DEFAULT: Self = Self;
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFunction for SumValues {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        vec![FeatureFunctionReturn {
            name: "sum_values".to_string(),
            value: sum_values(series),
        }]
    }
}

/// Aggregates and computes all minimal feature functions for a given time series.
#[derive(Default)]
pub struct MinimalFeatureSet {}

impl MinimalFeatureSet {
    pub fn new() -> Self {
        Self {}
    }
}

impl FeatureFunction for MinimalFeatureSet {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
        let mut results = Vec::new();

        results.extend(AbsoluteMaximum::DEFAULT.apply(series));
        results.extend(Mean::DEFAULT.apply(series));
        results.extend(Median::DEFAULT.apply(series));
        results.extend(Variance::DEFAULT.apply(series));
        results.extend(StandardDeviation::DEFAULT.apply(series));
        results.extend(Length::DEFAULT.apply(series));
        results.extend(Maximum::DEFAULT.apply(series));
        results.extend(Minimum::DEFAULT.apply(series));
        results.extend(RootMeanSquare::DEFAULT.apply(series));
        results.extend(SumValues::DEFAULT.apply(series));

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;

    #[test]
    fn test_absolute_maximum() {
        let series = [1.0, -3.0, 2.0];
        let result = AbsoluteMaximum::new().apply(&series);
        assert_float_eq!(result[0].value, 3.0);

        let series_pos = [1.0, 2.0, 0.5];
        let result = AbsoluteMaximum::new().apply(&series_pos);
        assert_float_eq!(result[0].value, 2.0);

        let series_zero = [0.0];
        let result = AbsoluteMaximum::new().apply(&series_zero);
        assert_float_eq!(result[0].value, 0.0);
    }

    #[test]
    fn test_mean() {
        let series = [1.0, 2.0, 3.0, 4.0, 5.0];
        let result = Mean::new().apply(&series);
        assert_float_eq!(result[0].value, 3.0);

        let single = [7.0];
        let result = Mean::new().apply(&single);
        assert_float_eq!(result[0].value, 7.0);

        let negative = [-1.0, 1.0, -1.0, 1.0];
        let result = Mean::new().apply(&negative);
        assert_float_eq!(result[0].value, 0.0);

        let zeros = [0.0, 0.0, 0.0];
        let result = Mean::new().apply(&zeros);
        assert_float_eq!(result[0].value, 0.0);
    }

    #[test]
    fn test_median() {
        let odd = [1.0, 2.0, 3.0, 4.0, 5.0];
        let result = Median::new().apply(&odd);
        assert_float_eq!(result[0].value, 3.0);

        let even = [1.0, 2.0, 3.0, 4.0];
        let result = Median::new().apply(&even);
        assert_float_eq!(result[0].value, 2.5);

        let single = [7.0];
        let result = Median::new().apply(&single);
        assert_float_eq!(result[0].value, 7.0);

        let negative = [-5.0, -1.0, 0.0, 2.0, 10.0];
        let result = Median::new().apply(&negative);
        assert_float_eq!(result[0].value, 0.0);

        let unsorted = [3.0, 1.0, 4.0, 1.0, 5.0, 9.0];
        let result = Median::new().apply(&unsorted);
        assert_float_eq!(result[0].value, 3.5);
    }

    #[test]
    fn test_variance() {
        let series = [1.0, 2.0, 3.0, 4.0, 5.0];
        let result = Variance::new().apply(&series);
        assert_float_eq!(result[0].value, 2.0);

        let constant = [3.0, 3.0, 3.0];
        let result = Variance::new().apply(&constant);
        assert_float_eq!(result[0].value, 0.0);

        let single = [5.0];
        let result = Variance::new().apply(&single);
        assert_float_eq!(result[0].value, 0.0);

        let negative = [-2.0, -1.0, 0.0, 1.0, 2.0];
        let result = Variance::new().apply(&negative);
        assert_float_eq!(result[0].value, 2.0);
    }

    #[test]
    fn test_standard_deviation() {
        let series = [1.0, 2.0, 3.0, 4.0, 5.0];
        let result = StandardDeviation::new().apply(&series);
        assert_float_eq!(result[0].value, 2.0f64.sqrt());

        let constant = [3.0, 3.0, 3.0];
        let result = StandardDeviation::new().apply(&constant);
        assert_float_eq!(result[0].value, 0.0);

        let single = [5.0];
        let result = StandardDeviation::new().apply(&single);
        assert_float_eq!(result[0].value, 0.0);
    }

    #[test]
    fn test_length() {
        let series = [1.0, 2.0, 3.0];
        let result = Length::new().apply(&series);
        assert_float_eq!(result[0].value, 3.0);

        let empty: [f64; 0] = [];
        let result = Length::new().apply(&empty);
        assert_float_eq!(result[0].value, 0.0);

        let single = [1.0];
        let result = Length::new().apply(&single);
        assert_float_eq!(result[0].value, 1.0);
    }

    #[test]
    fn test_maximum() {
        let series = [1.0, 5.0, 3.0, 9.0, 2.0];
        let result = Maximum::new().apply(&series);
        assert_float_eq!(result[0].value, 9.0);

        let negative = [-5.0, -1.0, -10.0];
        let result = Maximum::new().apply(&negative);
        assert_float_eq!(result[0].value, -1.0);

        let single = [7.0];
        let result = Maximum::new().apply(&single);
        assert_float_eq!(result[0].value, 7.0);
    }

    #[test]
    fn test_minimum() {
        let series = [1.0, 5.0, 3.0, 9.0, 2.0];
        let result = Minimum::new().apply(&series);
        assert_float_eq!(result[0].value, 1.0);

        let negative = [-5.0, -1.0, -10.0];
        let result = Minimum::new().apply(&negative);
        assert_float_eq!(result[0].value, -10.0);

        let single = [7.0];
        let result = Minimum::new().apply(&single);
        assert_float_eq!(result[0].value, 7.0);
    }

    #[test]
    fn test_root_mean_square() {
        let series = [3.0, 4.0];
        let expected: f64 = ((9.0_f64 + 16.0) / 2.0).sqrt();
        let result = RootMeanSquare::new().apply(&series);
        assert_float_eq!(result[0].value, expected);

        let zeros = [0.0, 0.0, 0.0];
        let result = RootMeanSquare::new().apply(&zeros);
        assert_float_eq!(result[0].value, 0.0);

        let single = [5.0];
        let result = RootMeanSquare::new().apply(&single);
        assert_float_eq!(result[0].value, 5.0);

        let negative = [-3.0, -4.0];
        let result = RootMeanSquare::new().apply(&negative);
        assert_float_eq!(result[0].value, expected);
    }

    #[test]
    fn test_sum_values() {
        let series = [1.0, 2.0, 3.0, 4.0];
        let result = SumValues::new().apply(&series);
        assert_float_eq!(result[0].value, 10.0);

        let negative = [1.0, -2.0, 3.0, -4.0];
        let result = SumValues::new().apply(&negative);
        assert_float_eq!(result[0].value, -2.0);

        let empty: [f64; 0] = [];
        let result = SumValues::new().apply(&empty);
        assert_float_eq!(result[0].value, 0.0);

        let single = [5.0];
        let result = SumValues::new().apply(&single);
        assert_float_eq!(result[0].value, 5.0);

        let zeros = [0.0, 0.0, 0.0];
        let result = SumValues::new().apply(&zeros);
        assert_float_eq!(result[0].value, 0.0);
    }
}
