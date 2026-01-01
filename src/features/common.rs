pub struct FeatureFunctionReturn {
    pub name: String,
    pub value: f64,
}

pub trait FeatureFunction {
    fn apply(&self, series: &[f64]) -> Vec<FeatureFunctionReturn>;
}
