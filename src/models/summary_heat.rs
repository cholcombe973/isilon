#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryHeat {
    #[serde(rename = "heat")]
    pub heat: Option<Vec<::models::SummaryHeatHeatItem>>,
}
