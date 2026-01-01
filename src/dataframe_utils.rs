//! DataFrame utility functions for feature extraction operations.
//!
//! This module provides helper functions for extracting features from Polars DataFrames.
//! It handles the core operations of partitioning, sorting, and applying feature functions
//! to time series data grouped by identifier columns.

use polars::prelude::*;

use crate::features::{
    common::FeatureFunction,
    minimal::{
        AbsoluteMaximum, Length, Maximum, Mean, Median, Minimum, RootMeanSquare, StandardDeviation,
        SumValues, Variance,
    },
};

/// Validates input DataFrame and identifies feature columns.
///
/// # Parameters
///
/// * `df` - Input DataFrame to validate
/// * `column_id` - Name of the ID column for grouping
/// * `column_sort` - Name of the sort column for ordering
///
/// # Returns
///
/// Result containing tuple of (validated DataFrame, feature column names)
///
/// # Errors
///
/// Returns error if required columns are missing from the DataFrame
pub fn validate_and_prepare_dataframe(
    df: &DataFrame,
    column_id: &str,
    column_sort: &str,
) -> Result<(DataFrame, Vec<String>), Box<dyn std::error::Error>> {
    // Check that required columns exist
    let columns = df.get_column_names();

    if !columns.iter().any(|col| col.as_str() == column_id) {
        return Err(format!("Column '{}' not found in DataFrame", column_id).into());
    }

    if !columns.iter().any(|col| col.as_str() == column_sort) {
        return Err(format!("Column '{}' not found in DataFrame", column_sort).into());
    }

    // Get feature columns (all except ID and sort columns)
    let feature_columns: Vec<String> = columns
        .iter()
        .filter(|&&col| col.as_str() != column_id && col.as_str() != column_sort)
        .map(|&col| col.to_string())
        .collect();

    if feature_columns.is_empty() {
        return Err("No feature columns found after excluding ID and sort columns".into());
    }

    Ok((df.clone(), feature_columns))
}

/// Partitions DataFrame by unique values in the ID column.
///
/// # Parameters
///
/// * `df` - Input DataFrame to partition
/// * `column_id` - Name of the ID column for grouping
///
/// # Returns
///
/// Result containing vector of (id_value, partitioned_dataframe) tuples
///
/// # Errors
///
/// Returns error if partitioning fails
pub fn get_partitioned_dataframes(
    df: &DataFrame,
    column_id: &str,
) -> Result<Vec<(String, DataFrame)>, Box<dyn std::error::Error>> {
    // Get unique ID values first
    let id_series = df.column(column_id)?.cast(&DataType::String)?;
    let id_values = id_series
        .unique()?
        .str()?
        .into_no_null_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let mut result = Vec::new();

    for id_value in id_values {
        let id_str = id_value.to_string();

        // Filter DataFrame for this specific ID value
        let filtered_df = df.filter(&df.column(column_id)?.str()?.equal(id_value))?;

        result.push((id_str, filtered_df));
    }

    Ok(result)
}

/// Applies all feature extraction functions to a single time series.
///
/// # Parameters
///
/// * `series` - Time series data as slice of f64 values
/// * `column_name` - Name of the original column (for naming features)
///
/// # Returns
///
/// Vector of (feature_name, feature_value) tuples with naming convention {column_name}__{feature_name}
pub fn apply_features_to_series(series: &[f64], column_name: &str) -> Vec<(String, f64)> {
    let mut results = Vec::new();

    // All available feature functions from the minimal module
    let feature_functions: Vec<Box<dyn FeatureFunction>> = vec![
        Box::new(AbsoluteMaximum::new()),
        Box::new(Length::new()),
        Box::new(Maximum::new()),
        Box::new(Mean::new()),
        Box::new(Median::new()),
        Box::new(Minimum::new()),
        Box::new(RootMeanSquare::new()),
        Box::new(StandardDeviation::new()),
        Box::new(SumValues::new()),
        Box::new(Variance::new()),
    ];

    for feature_fn in feature_functions {
        let feature_results = feature_fn.apply(series);
        for feature in feature_results {
            let feature_name = format!("{}__{}", column_name, feature.name);
            results.push((feature_name, feature.value));
        }
    }

    results
}

/// Processes a single group (partition) and extracts features from all feature columns.
///
/// # Parameters
///
/// * `group_df` - DataFrame for a single group
/// * `id` - ID value for this group
/// * `column_sort` - Name of the column to sort by
/// * `feature_columns` - List of feature column names to process
///
/// # Returns
///
/// Result containing tuple of (id, vector_of_feature_results)
///
/// # Errors
///
/// Returns error if sorting or feature extraction fails
pub fn process_single_group(
    group_df: &DataFrame,
    id: &str,
    column_sort: &str,
    feature_columns: &[String],
) -> Result<(String, Vec<(String, f64)>), Box<dyn std::error::Error>> {
    // Sort the group DataFrame by the sort column
    let sorted_df = group_df.sort([column_sort], Default::default())?;

    let mut all_features = Vec::new();

    // Process each feature column
    for column_name in feature_columns {
        let series = sorted_df.column(column_name)?;
        // Convert Series to Vec<f64>
        let f64_series = series.f64()?;
        let time_series_data: Vec<f64> = f64_series.into_no_null_iter().collect();

        // Apply features to this time series
        let features = apply_features_to_series(&time_series_data, column_name);
        all_features.extend(features);
    }

    Ok((id.to_string(), all_features))
}

/// Assembles the final result DataFrame from processed group data.
///
/// # Parameters
///
/// * `group_results` - Vector of (id, feature_results) tuples from processed groups
/// * `column_id` - Name of the ID column for the result DataFrame
///
/// # Returns
///
/// Result containing assembled DataFrame with ID and feature columns
///
/// # Errors
///
/// Returns error if DataFrame assembly fails
pub fn assemble_result_dataframe(
    group_results: Vec<(String, Vec<(String, f64)>)>,
    column_id: &str,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    if group_results.is_empty() {
        return Err("No group results to assemble".into());
    }

    // Collect all unique feature names
    let mut feature_names = std::collections::HashSet::new();
    for (_, features) in &group_results {
        for (name, _) in features {
            feature_names.insert(name.clone());
        }
    }

    let mut feature_names: Vec<String> = feature_names.into_iter().collect();
    feature_names.sort(); // Sort for consistent column ordering

    // Create ID column
    let id_values: Vec<String> = group_results.iter().map(|(id, _)| id.clone()).collect();
    let id_series = Series::new(column_id.into(), id_values);

    // Create feature columns
    let mut columns = vec![id_series];

    for feature_name in &feature_names {
        let feature_values: Vec<f64> = group_results
            .iter()
            .map(|(_, features)| {
                features
                    .iter()
                    .find(|(name, _)| name == feature_name)
                    .map(|(_, value)| *value)
                    .unwrap_or(0.0) // Default value if feature not found
            })
            .collect();

        let feature_series = Series::new(feature_name.into(), feature_values);
        columns.push(feature_series);
    }

    Ok(DataFrame::new(columns)?.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    #[test]
    fn test_validate_and_prepare_dataframe_success() {
        let df = df!(
            "id" => &[1, 2, 3],
            "timestamp" => &[1, 2, 3],
            "value1" => &[1.0, 2.0, 3.0],
            "value2" => &[4.0, 5.0, 6.0]
        )
        .unwrap();

        let result = validate_and_prepare_dataframe(&df, "id", "timestamp");
        assert!(result.is_ok());

        let (_, feature_columns) = result.unwrap();
        assert_eq!(feature_columns, vec!["value1", "value2"]);
    }

    #[test]
    fn test_validate_and_prepare_dataframe_missing_id_column() {
        let df = df!(
            "timestamp" => &[1, 2, 3],
            "value1" => &[1.0, 2.0, 3.0]
        )
        .unwrap();

        let result = validate_and_prepare_dataframe(&df, "id", "timestamp");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_and_prepare_dataframe_no_feature_columns() {
        let df = df!(
            "id" => &[1, 2, 3],
            "timestamp" => &[1, 2, 3]
        )
        .unwrap();

        let result = validate_and_prepare_dataframe(&df, "id", "timestamp");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_partitioned_dataframes() {
        let df = df!(
            "id" => &["A", "A", "B", "B"],
            "timestamp" => &[1, 2, 1, 2],
            "value" => &[1.0, 2.0, 3.0, 4.0]
        )
        .unwrap();

        let result = get_partitioned_dataframes(&df, "id");
        assert!(result.is_ok());

        let partitions = result.unwrap();
        assert_eq!(partitions.len(), 2);

        // Check that we have both "A" and "B" groups
        let ids: Vec<String> = partitions.iter().map(|(id, _)| id.clone()).collect();
        assert!(ids.contains(&"A".to_string()));
        assert!(ids.contains(&"B".to_string()));
    }

    #[test]
    fn test_apply_features_to_series() {
        let series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let results = apply_features_to_series(&series, "test_col");

        assert!(!results.is_empty());

        // Check naming convention
        for (name, _) in &results {
            assert!(name.starts_with("test_col__"));
        }

        // Check that we get expected features
        let feature_names: Vec<String> = results.iter().map(|(name, _)| name.clone()).collect();
        assert!(feature_names.iter().any(|name| name.contains("mean")));
        assert!(feature_names.iter().any(|name| name.contains("length")));
    }

    #[test]
    fn test_process_single_group() {
        let group_df = df!(
            "id" => &["A", "A"],
            "timestamp" => &[2, 1], // Unsorted to test sorting
            "value1" => &[2.0, 1.0]
        )
        .unwrap();

        let feature_columns = vec!["value1".to_string()];
        let result = process_single_group(&group_df, "A", "timestamp", &feature_columns);

        assert!(result.is_ok());

        let (id, features) = result.unwrap();
        assert_eq!(id, "A");
        assert!(!features.is_empty());

        // Check naming convention
        for (name, _) in &features {
            assert!(name.starts_with("value1__"));
        }
    }

    #[test]
    fn test_assemble_result_dataframe() {
        let group_results = vec![
            (
                "A".to_string(),
                vec![
                    ("value1__mean".to_string(), 2.0),
                    ("value1__length".to_string(), 3.0),
                ],
            ),
            (
                "B".to_string(),
                vec![
                    ("value1__mean".to_string(), 4.0),
                    ("value1__length".to_string(), 2.0),
                ],
            ),
        ];

        let result = assemble_result_dataframe(group_results, "id");
        assert!(result.is_ok());

        let df = result.unwrap();
        assert_eq!(df.width(), 3); // id + 2 feature columns
        assert_eq!(df.height(), 2); // 2 groups

        let columns = df.get_column_names();
        assert!(columns.contains(&"id"));
        assert!(columns.contains(&"value1__mean"));
        assert!(columns.contains(&"value1__length"));
    }

    #[test]
    fn test_assemble_result_dataframe_empty() {
        let group_results: Vec<(String, Vec<(String, f64)>)> = vec![];
        let result = assemble_result_dataframe(group_results, "id");
        assert!(result.is_err());
    }
}
