/// Calculates the absolute energy of the time series, which is the sum of squared values.
pub fn absolute_energy(series: &[f64]) -> f64 {
    series.iter().map(|&x| x * x).sum()
}

/// Calculates the highest absolute value in the time series.
pub fn absolute_maximum(series: &[f64]) -> f64 {
    series
        .iter()
        .map(|&x| x.abs())
        .fold(f64::NEG_INFINITY, f64::max)
}

/// Calculates the sum of absolute changes between consecutive values in the time series.
pub fn absolute_sum_of_changes(series: &[f64]) -> f64 {
    series.windows(2).map(|w| (w[1] - w[0]).abs()).sum()
}

pub fn mean(series: &[f64]) -> f64 {
    if series.is_empty() {
        return f64::NAN;
    }
    series.iter().sum::<f64>() / series.len() as f64
}

pub fn median(series: &[f64]) -> f64 {
    if series.is_empty() {
        return f64::NAN;
    }
    let mut sorted = series.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted.len();
    if len.is_multiple_of(2) {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    } else {
        sorted[len / 2]
    }
}

pub fn variance(series: &[f64]) -> f64 {
    if series.is_empty() {
        return f64::NAN;
    }
    let m = mean(series);
    series.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / series.len() as f64
}

pub fn standard_deviation(series: &[f64]) -> f64 {
    variance(series).sqrt()
}

pub fn length(series: &[f64]) -> usize {
    series.len()
}

pub fn maximum(series: &[f64]) -> f64 {
    if series.is_empty() {
        return f64::NAN;
    }
    series.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

pub fn minimum(series: &[f64]) -> f64 {
    if series.is_empty() {
        return f64::NAN;
    }
    series.iter().copied().fold(f64::INFINITY, f64::min)
}

pub fn root_mean_square(series: &[f64]) -> f64 {
    if series.is_empty() {
        return f64::NAN;
    }
    (series.iter().map(|&x| x * x).sum::<f64>() / series.len() as f64).sqrt()
}

pub fn sum_values(series: &[f64]) -> f64 {
    series.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;

    use super::*;

    #[test]
    fn test_absolute_energy() {
        let series = [1.0, 2.0, 3.0];
        assert_float_eq!(absolute_energy(&series), 14.0);

        let empty: [f64; 0] = [];
        assert_float_eq!(absolute_energy(&empty), 0.0);

        let series_neg = [-1.0, -2.0];
        assert_float_eq!(absolute_energy(&series_neg), 5.0);
    }

    #[test]
    fn test_absolute_maximum() {
        let series = [1.0, -3.0, 2.0];
        assert_float_eq!(absolute_maximum(&series), 3.0);

        let series_pos = [1.0, 2.0, 0.5];
        assert_float_eq!(absolute_maximum(&series_pos), 2.0);

        let series_zero = [0.0];
        assert_float_eq!(absolute_maximum(&series_zero), 0.0);
    }

    #[test]
    fn test_absolute_sum_of_changes() {
        let series = [1.0, 3.0, 2.0, 5.0];
        assert_float_eq!(absolute_sum_of_changes(&series), 6.0);

        let single = [1.0];
        assert_float_eq!(absolute_sum_of_changes(&single), 0.0);

        let empty: [f64; 0] = [];
        assert_float_eq!(absolute_sum_of_changes(&empty), 0.0);

        let constant = [2.0, 2.0, 2.0];
        assert_float_eq!(absolute_sum_of_changes(&constant), 0.0);
    }

    #[test]
    fn test_mean() {
        let series = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_float_eq!(mean(&series), 3.0);

        let single = [7.0];
        assert_float_eq!(mean(&single), 7.0);

        let negative = [-1.0, 1.0, -1.0, 1.0];
        assert_float_eq!(mean(&negative), 0.0);

        let zeros = [0.0, 0.0, 0.0];
        assert_float_eq!(mean(&zeros), 0.0);

        let empty: [f64; 0] = [];
        assert!(mean(&empty).is_nan());
    }

    #[test]
    fn test_median() {
        let odd = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_float_eq!(median(&odd), 3.0);

        let even = [1.0, 2.0, 3.0, 4.0];
        assert_float_eq!(median(&even), 2.5);

        let single = [7.0];
        assert_float_eq!(median(&single), 7.0);

        let negative = [-5.0, -1.0, 0.0, 2.0, 10.0];
        assert_float_eq!(median(&negative), 0.0);

        let unsorted = [3.0, 1.0, 4.0, 1.0, 5.0, 9.0];
        assert_float_eq!(median(&unsorted), 3.5);

        let empty: [f64; 0] = [];
        assert!(median(&empty).is_nan());
    }

    #[test]
    fn test_variance() {
        let series = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_float_eq!(variance(&series), 2.0);

        let constant = [3.0, 3.0, 3.0];
        assert_float_eq!(variance(&constant), 0.0);

        let single = [5.0];
        assert_float_eq!(variance(&single), 0.0);

        let negative = [-2.0, -1.0, 0.0, 1.0, 2.0];
        assert_float_eq!(variance(&negative), 2.0);

        let empty: [f64; 0] = [];
        assert!(variance(&empty).is_nan());
    }

    #[test]
    fn test_standard_deviation() {
        let series = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_float_eq!(standard_deviation(&series), 2.0f64.sqrt());

        let constant = [3.0, 3.0, 3.0];
        assert_float_eq!(standard_deviation(&constant), 0.0);

        let single = [5.0];
        assert_float_eq!(standard_deviation(&single), 0.0);

        let empty: [f64; 0] = [];
        assert!(standard_deviation(&empty).is_nan());
    }

    #[test]
    fn test_length() {
        let series = [1.0, 2.0, 3.0];
        assert_eq!(length(&series), 3);

        let empty: [f64; 0] = [];
        assert_eq!(length(&empty), 0);

        let single = [1.0];
        assert_eq!(length(&single), 1);
    }

    #[test]
    fn test_maximum() {
        let series = [1.0, 5.0, 3.0, 9.0, 2.0];
        assert_float_eq!(maximum(&series), 9.0);

        let negative = [-5.0, -1.0, -10.0];
        assert_float_eq!(maximum(&negative), -1.0);

        let single = [7.0];
        assert_float_eq!(maximum(&single), 7.0);

        let empty: [f64; 0] = [];
        assert!(maximum(&empty).is_nan());
    }

    #[test]
    fn test_minimum() {
        let series = [1.0, 5.0, 3.0, 9.0, 2.0];
        assert_float_eq!(minimum(&series), 1.0);

        let negative = [-5.0, -1.0, -10.0];
        assert_float_eq!(minimum(&negative), -10.0);

        let single = [7.0];
        assert_float_eq!(minimum(&single), 7.0);

        let empty: [f64; 0] = [];
        assert!(minimum(&empty).is_nan());
    }

    #[test]
    fn test_root_mean_square() {
        let series = [3.0, 4.0];
        let expected: f64 = ((9.0_f64 + 16.0) / 2.0).sqrt();
        assert_float_eq!(root_mean_square(&series), expected);

        let zeros = [0.0, 0.0, 0.0];
        assert_float_eq!(root_mean_square(&zeros), 0.0);

        let single = [5.0];
        assert_float_eq!(root_mean_square(&single), 5.0);

        let negative = [-3.0, -4.0];
        assert_float_eq!(root_mean_square(&negative), expected);

        let empty: [f64; 0] = [];
        assert!(root_mean_square(&empty).is_nan());
    }

    #[test]
    fn test_sum_values() {
        let series = [1.0, 2.0, 3.0, 4.0];
        assert_float_eq!(sum_values(&series), 10.0);

        let negative = [1.0, -2.0, 3.0, -4.0];
        assert_float_eq!(sum_values(&negative), -2.0);

        let empty: [f64; 0] = [];
        assert_float_eq!(sum_values(&empty), 0.0);

        let single = [5.0];
        assert_float_eq!(sum_values(&single), 5.0);

        let zeros = [0.0, 0.0, 0.0];
        assert_float_eq!(sum_values(&zeros), 0.0);
    }
}
