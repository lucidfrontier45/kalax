//! Functional API for minimal feature extraction.

/// Calculates the absolute maximum value in a time series.
pub fn absolute_maximum(series: &[f64]) -> f64 {
    series
        .iter()
        .map(|&x| x.abs())
        .fold(f64::NEG_INFINITY, f64::max)
}

/// Calculates the arithmetic mean of a time series.
pub fn mean(series: &[f64]) -> f64 {
    series.iter().sum::<f64>() / series.len() as f64
}

/// Calculates the median value of a time series.
pub fn median(series: &[f64]) -> f64 {
    let mut sorted = series.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted.len();
    if len.is_multiple_of(2) {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    } else {
        sorted[len / 2]
    }
}

/// Calculates the variance of a time series.
pub fn variance(series: &[f64]) -> f64 {
    let m = mean(series);
    series.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / series.len() as f64
}

/// Calculates the standard deviation of a time series.
pub fn standard_deviation(series: &[f64]) -> f64 {
    variance(series).sqrt()
}

/// Returns the length (number of elements) of a time series.
pub fn length(series: &[f64]) -> usize {
    series.len()
}

/// Calculates the maximum value in a time series.
pub fn maximum(series: &[f64]) -> f64 {
    series.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

/// Calculates the minimum value in a time series.
pub fn minimum(series: &[f64]) -> f64 {
    series.iter().copied().fold(f64::INFINITY, f64::min)
}

/// Calculates the root mean square (RMS) of a time series.
pub fn root_mean_square(series: &[f64]) -> f64 {
    (series.iter().map(|&x| x * x).sum::<f64>() / series.len() as f64).sqrt()
}

/// Calculates the sum of all values in a time series.
pub fn sum_values(series: &[f64]) -> f64 {
    series.iter().sum()
}
