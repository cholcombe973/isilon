

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOperation {
  /// The name of the operation.
  #[serde(rename = "operation")]
  operation: String
}

