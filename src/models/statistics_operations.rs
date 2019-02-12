#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperations {
    #[serde(rename = "operations")]
    pub operations: Option<Vec<crate::models::StatisticsOperation>>,
}
