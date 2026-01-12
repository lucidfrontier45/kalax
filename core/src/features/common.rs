/// Represents the result of a feature extraction operation.
///
/// Contains the name of the feature and its computed value.
/// This struct is used to return feature extraction results in a standardized format.
///
/// # Fields
///
/// * `name` - The name of the extracted feature (e.g., "mean", "variance")
/// * `value` - The computed numerical value of the feature
///
/// # Examples
///
/// ```
/// use kalax::features::common::FeatureFunctionReturn;
///
/// let result = FeatureFunctionReturn {
///     name: "mean".to_string(),
///     value: 42.5,
/// };
///
/// assert_eq!(result.name, "mean");
/// assert_eq!(result.value, 42.5);
/// ```
pub struct FeatureFunctionReturn {
    pub name: String,
    pub value: f64,
}

/// Trait for feature extraction functions that operate on time series data.
///
/// This trait defines a common interface for extracting features from time series data.
/// Implementations of this trait can compute one or more features from a given time series
/// and return the results as a vector of `FeatureFunctionReturn` values.
///
/// This trait is primarily used for the OOP-style API where feature extraction functions
/// are implemented as structs that can be instantiated and applied to time series data.
///
/// # Examples
///
/// ```
/// use kalax::features::common::{FeatureFunction, FeatureFunctionReturn};
///
/// struct MeanFeature;
///
/// impl FeatureFunction for MeanFeature {
///     fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn> {
///         if series.is_empty() {
///             return vec![];
///         }
///         let mean_value = series.iter().sum::<f64>() / series.len() as f64;
///         vec![FeatureFunctionReturn {
///             name: "mean".to_string(),
///             value: mean_value,
///         }]
///     }
/// }
///
/// let feature = MeanFeature;
/// let series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let results = feature.apply(&series);
///
/// assert_eq!(results.len(), 1);
/// assert_eq!(results[0].name, "mean");
/// assert_eq!(results[0].value, 3.0);
/// ```
pub trait FeatureFunction {
    /// Apply the feature extraction function to a time series.
    ///
    /// # Parameters
    ///
    /// * `series` - A slice of f64 values representing the time series data
    ///
    /// # Returns
    ///
    /// A vector of `FeatureFunctionReturn` containing the computed features.
    /// The vector may contain multiple results if the function computes several related features.
    ///
    /// # Notes
    ///
    /// - Implementations should handle empty input series appropriately
    /// - The function should not modify the input series
    /// - Feature names should be descriptive and consistent across implementations
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn>;
}
