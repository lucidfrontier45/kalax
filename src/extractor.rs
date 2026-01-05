use std::collections::HashMap;

use rayon::prelude::*;

use crate::features::{common::FeatureFunction as _, minimal::MinimalFeatureSet};

/// Extracts features from the given data.
/// 
/// # Arguments
/// * `data` - A slice of HashMaps where each HashMap represents multiple columns
///
/// # Returns
/// A vector where each element corresponds to an input `HashMap`. Each result
/// contains column names mapped to their extracted features, with each feature
/// map using `feature name -> feature value`.
pub fn extract_features(
    data: &[HashMap<String, Vec<f64>>],
) -> Vec<HashMap<String, HashMap<String, f64>>> {
    // apply minimal feature set

    let feature_func = MinimalFeatureSet::new();
    data.par_iter()
        .map(|row| {
            row.par_iter()
                .map(|(col_name, series)| {
                    let features = feature_func.apply(series);
                    let feature_map: HashMap<String, f64> =
                        features.into_iter().map(|f| (f.name, f.value)).collect();
                    (col_name.clone(), feature_map)
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serdeio::read_records_from_file;

    use crate::assert_float_eq;

    use super::extract_features;

    #[derive(Deserialize)]
    struct DataRecord {
        date: String,
        close: f64,
    }

    #[allow(clippy::get_first)]
    fn read_tsfresh_result(path: &str) -> HashMap<String, f64> {
        let contents = std::fs::read_to_string(path).expect("failed to read tsfresh result file");
        let lines: Vec<&str> = contents.lines().collect();

        let header_line = lines.get(0).expect("missing header line");
        let values_line = lines.get(1).expect("missing values line");

        let headers: Vec<String> = header_line
            .split(',')
            .map(|h| h.strip_prefix("close__").unwrap_or(h).to_string())
            .collect();

        let values: Vec<f64> = values_line
            .split(',')
            .map(|v| v.trim().parse::<f64>().expect("failed to parse value"))
            .collect();

        headers.into_iter().zip(values).collect()
    }

    #[test]
    fn test_minimal_extractor() {
        let mut records: Vec<DataRecord> = read_records_from_file("test_data/sp500_raw.csv")
            .expect("failed to read sp500 raw data");
        records.sort_by(|a, b| a.date.cmp(&b.date));
        // convert to HashMap<String, Vec<f64>>
        let x = HashMap::from([(
            "close".to_string(),
            records.iter().map(|rec| rec.close).collect(),
        )]);
        let y_true = read_tsfresh_result("test_data/sp500_tsfresh_features.csv");
        let y = extract_features(&[x]);
        let y_close = &y[0]["close"];
        for (feature_name, true_value) in y_true {
            let extracted_value = y_close
                .get(&feature_name)
                .copied()
                .expect("missing extracted feature");
            assert_float_eq!(extracted_value, true_value, 1e-6);
        }
    }
}
