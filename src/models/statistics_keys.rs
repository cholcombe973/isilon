

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsKeys {
  #[serde(rename = "keys")]
  keys: Option<Vec<::models::StatisticsKey>>
}

