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
}
