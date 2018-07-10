

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperations {
  #[serde(rename = "operations")]
  operations: Option<Vec<::models::StatisticsOperation>>
}

