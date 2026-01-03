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
pub fn validate_dataframe(
    df: &DataFrame,
    column_id: &str,
    column_sort: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

    Ok(feature_columns)
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
    df: DataFrame,
    column_id: &str,
) -> Result<Vec<(String, DataFrame)>, Box<dyn std::error::Error>> {
    let partitioned = df
        .partition_by([column_id], true)?
        .into_iter()
        .map(|mut group_df| {
            // all raw values in the ID column are the same for this group
            // just take the first value as the ID and remove the column from the group DataFrame
            let id_series = group_df.column(column_id).unwrap();
            let id_value = id_series.get(0).unwrap().extract_str().unwrap().to_string();
            group_df.drop_in_place(column_id).unwrap();
            (id_value, group_df)
        })
        .collect();
    Ok(partitioned)
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
    let feature_functions: Vec<&dyn FeatureFunction> = vec![
        &AbsoluteMaximum::DEFAULT,
        &Length::DEFAULT,
        &Maximum::DEFAULT,
        &Mean::DEFAULT,
        &Median::DEFAULT,
        &Minimum::DEFAULT,
        &RootMeanSquare::DEFAULT,
        &StandardDeviation::DEFAULT,
        &SumValues::DEFAULT,
        &Variance::DEFAULT,
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
) -> Result<(String, Vec<(String, f64)>), PolarsError> {
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
    let id_series = Column::from(Series::new(column_id.into(), id_values));

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
                    .unwrap_or(f64::NAN)
            })
            .collect();

        let feature_series = Series::new(feature_name.into(), feature_values);
        let column = Column::from(feature_series);
        columns.push(column);
    }

    let result_df = DataFrame::new(columns)?;
    Ok(result_df)
}

/// Extracts features from a DataFrame by grouping, sorting, and applying statistical features.
///
/// This function splits the input DataFrame into multiple DataFrames by the values in `column_id`,
/// sorts each group by `column_sort`, applies all available feature extraction functions to each
/// group's feature columns (all columns except `column_id` and `column_sort`), and collects the
/// results into a new DataFrame.
///
/// # Parameters
///
/// * `df` - Input DataFrame to process
/// * `column_id` - Name of the column to group by (string/identifier column)
/// * `column_sort` - Name of the column to sort each group by (typically time/timestamp)
///
/// # Returns
///
/// Result containing a new DataFrame with:
/// - The original `column_id` values (renamed to match the parameter name)
/// - Feature columns with naming convention: `{original_column_name}__{feature_name}`
///
/// # Example
///
/// ```rust
/// use kalax::polars::extract_features;
/// use polars::prelude::*;
///
/// let df = df!(
///     "id" => &["A", "A", "B", "B"],
///     "time" => &[1, 2, 1, 2],
///     "value1" => &[1.0, 2.0, 3.0, 4.0],
///     "value2" => &[5.0, 6.0, 7.0, 8.0]
/// ).unwrap();
///
/// let result = extract_features(df, "id", "time")?;
/// // Result will have columns: id, value1__mean, value1__length, value1__variance, etc.
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Notes
///
/// - All feature columns (except `column_id` and `column_sort`) are assumed to be f64 type
/// - Empty groups are skipped from the output
/// - Features are extracted using all functions from the minimal module
pub fn extract_features(
    df: DataFrame,
    column_id: &str,
    column_sort: &str,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    // 1. Validate input and get feature columns
    let feature_columns = validate_dataframe(&df, column_id, column_sort)?;

    // 2. Get partitioned DataFrames by ID
    let partitions = get_partitioned_dataframes(df, column_id)?;

    // 3. Process each partition and extract features
    let mut group_results = Vec::new();
    for (id, group_df) in partitions {
        match process_single_group(&group_df, &id, column_sort, &feature_columns) {
            Ok(result) => {
                group_results.push(result);
            }
            Err(e) => {
                // Log errors to avoid silently skipping non-empty groups
                eprintln!("Warning: failed to process group '{}': {}", id, e);
                // Skip groups that fail processing (including empty groups) as per requirement
            }
        }
    }

    // 4. Assemble final result DataFrame
    assemble_result_dataframe(group_results, column_id)
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::assert_float_eq;

    use super::*;

    #[test]
    fn test_validate_and_prepare_dataframe_success() {
        let df = df!(
            "id" => &[1, 2, 3],
            "timestamp" => &[1, 2, 3],
            "value1" => &[1.0, 2.0, 3.0],
            "value2" => &[4.0, 5.0, 6.0]
        )
        .unwrap();

        let result = validate_dataframe(&df, "id", "timestamp");
        assert!(result.is_ok());

        let feature_columns = result.unwrap();
        assert_eq!(feature_columns, vec!["value1", "value2"]);
    }

    #[test]
    fn test_validate_and_prepare_dataframe_missing_id_column() {
        let df = df!(
            "timestamp" => &[1, 2, 3],
            "value1" => &[1.0, 2.0, 3.0]
        )
        .unwrap();

        let result = validate_dataframe(&df, "id", "timestamp");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_and_prepare_dataframe_no_feature_columns() {
        let df = df!(
            "id" => &[1, 2, 3],
            "timestamp" => &[1, 2, 3]
        )
        .unwrap();

        let result = validate_dataframe(&df, "id", "timestamp");
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

        let result = get_partitioned_dataframes(df, "id");
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
        for target in ["id", "value1__mean", "value1__length"] {
            let small_str = PlSmallStr::from_str(target);
            assert!(columns.contains(&&small_str));
        }
    }

    #[test]
    fn test_assemble_result_dataframe_empty() {
        let group_results: Vec<(String, Vec<(String, f64)>)> = vec![];
        let result = assemble_result_dataframe(group_results, "id");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_features_integration() {
        // Create test DataFrame with multiple groups and columns
        let df = df!(
            "id" => &["A", "A", "A", "B", "B", "B"],
            "timestamp" => &[3, 1, 2, 6, 4, 5], // Unsorted to test sorting
            "value1" => &[3.0, 1.0, 2.0, 6.0, 4.0, 5.0],
            "value2" => &[8.0, 6.0, 7.0, 11.0, 9.0, 10.0]
        )
        .unwrap();

        let result = extract_features(df, "id", "timestamp");
        assert!(result.is_ok());

        let result_df = result.unwrap();

        // Check structure
        assert_eq!(result_df.width(), 21); // id + 2 columns * 10 features each
        assert_eq!(result_df.height(), 2); // 2 groups (A and B)

        // Check that ID column exists
        assert!(
            result_df
                .get_column_names()
                .contains(&&PlSmallStr::from_str("id"))
        );

        // Check that feature columns follow naming convention
        let columns = result_df.get_column_names();
        let feature_columns: Vec<&str> = columns
            .iter()
            .filter(|&&col| col != "id")
            .map(|&col| col.as_str())
            .collect();

        // Should have features for both value1 and value2
        assert!(
            feature_columns
                .iter()
                .any(|col| col.starts_with("value1__"))
        );
        assert!(
            feature_columns
                .iter()
                .any(|col| col.starts_with("value2__"))
        );

        // Check specific expected features
        let expected_features = [
            "mean",
            "length",
            "variance",
            "standard_deviation",
            "minimum",
            "maximum",
            "absolute_maximum",
            "root_mean_square",
            "sum_values",
            "median",
        ];

        for feature in &expected_features {
            assert!(feature_columns.iter().any(|col| col.contains(feature)));
        }

        // Verify mean calculation (after sorting: A should have [1.0, 2.0, 3.0], B should have [4.0, 5.0, 6.0])
        let value1_mean_series = result_df.column("value1__mean").unwrap();
        let means = value1_mean_series
            .f64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<f64>>();

        // Group A: [1.0, 2.0, 3.0] -> mean = 2.0
        // Group B: [4.0, 5.0, 6.0] -> mean = 5.0
        assert!(means.contains(&2.0));
        assert!(means.contains(&5.0));

        // Check length calculation (all groups have 3 rows)
        let value1_length_series = result_df.column("value1__length").unwrap();
        let lengths = value1_length_series
            .f64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<f64>>();
        assert!(lengths.iter().all(|&len| len == 3.0));
    }

    #[test]
    fn test_extract_features_single_group() {
        let df = df!(
            "id" => &["A", "A", "A"],
            "timestamp" => &[1, 2, 3],
            "value" => &[1.0, 2.0, 3.0]
        )
        .unwrap();

        let result = extract_features(df, "id", "timestamp");
        assert!(result.is_ok());

        let result_df = result.unwrap();
        assert_eq!(result_df.width(), 11); // id + 10 features
        assert_eq!(result_df.height(), 1); // 1 group
    }

    #[test]
    fn test_extract_features_empty_result() {
        let df = df!(
            "id" => &["A"],
            "timestamp" => &[1],
            "value" => &[1.0]
        )
        .unwrap();

        let result = extract_features(df, "nonexistent", "timestamp");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_features_no_feature_columns() {
        let df = df!(
            "id" => &["A", "B"],
            "timestamp" => &[1, 2]
        )
        .unwrap();

        let result = extract_features(df, "id", "timestamp");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_features_single_value_groups() {
        // TODO: Handle the case where variance/std dev for single values should be 0
        // This test documents the current behavior
        let df = df!(
            "id" => &["A", "B"],
            "timestamp" => &[1, 1],
            "value" => &[5.0, 10.0]
        )
        .unwrap();

        let result = extract_features(df, "id", "timestamp");
        assert!(result.is_ok());

        let result_df = result.unwrap();
        let variance_series = result_df.column("value__variance").unwrap();
        let variances = variance_series
            .f64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<f64>>();

        // Both groups have single values, so variance should be 0
        assert!(variances.iter().all(|&var| var == 0.0));
    }

    #[test]
    fn test_extract_features_vs_tsfresh() {
        // 1. Read raw S&P 500 data
        let mut df = CsvReader::new(File::open("test_data/sp500_raw.csv").unwrap())
            .finish()
            .unwrap();

        // Add constant ID column for single group processing
        let height = df.height();
        let id_column = Series::new("id".into(), vec!["single_group"; height]);
        df.with_column(id_column).unwrap();

        // 2. Extract features using kalax
        let result = extract_features(df, "id", "date").unwrap();

        // 3. Load tsfresh reference features
        let tsfresh_df =
            CsvReader::new(File::open("test_data/sp500_tsfresh_features.csv").unwrap())
                .finish()
                .unwrap();

        // 4. Compare all features
        let features_to_compare = [
            ("close__sum_values", "close__sum_values"),
            ("close__median", "close__median"),
            ("close__mean", "close__mean"),
            ("close__length", "close__length"),
            ("close__standard_deviation", "close__standard_deviation"),
            ("close__variance", "close__variance"),
            ("close__root_mean_square", "close__root_mean_square"),
            ("close__maximum", "close__maximum"),
            ("close__absolute_maximum", "close__absolute_maximum"),
            ("close__minimum", "close__minimum"),
        ];

        for (kalax_col, tsfresh_col) in &features_to_compare {
            let kalax_val = result
                .column(kalax_col)
                .unwrap_or_else(|_| panic!("Column '{}' not found in kalax result", kalax_col))
                .f64()
                .unwrap()
                .get(0)
                .unwrap();

            let tsfresh_val = tsfresh_df
                .column(tsfresh_col)
                .unwrap_or_else(|_| {
                    panic!("Column '{}' not found in tsfresh reference", tsfresh_col)
                })
                .f64()
                .unwrap()
                .get(0)
                .unwrap();

            assert_float_eq!(kalax_val, tsfresh_val, 1e-5);
        }
    }
}
