use polars::frame::DataFrame;

pub mod dataframe_utils;
pub mod features;

// Re-export the main extract_features function from this module
// The function is defined in this file below

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
/// use kalax::extract_features;
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
    use crate::dataframe_utils::{
        assemble_result_dataframe, get_partitioned_dataframes, process_single_group,
        validate_and_prepare_dataframe,
    };

    // 1. Validate input and get feature columns
    let (validated_df, feature_columns) =
        validate_and_prepare_dataframe(&df, column_id, column_sort)?;

    // 2. Get partitioned DataFrames by ID
    let partitions = get_partitioned_dataframes(validated_df, column_id)?;

    // 3. Process each partition and extract features
    let mut group_results = Vec::new();
    for (id, group_df) in partitions {
        if let Ok(result) = process_single_group(&group_df, &id, column_sort, &feature_columns) {
            group_results.push(result);
        }
        // Skip empty groups as per requirement
    }

    // 4. Assemble final result DataFrame
    assemble_result_dataframe(group_results, column_id)
}

#[macro_use]
pub mod test_utils;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use polars::prelude::*;

    use super::*;

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
