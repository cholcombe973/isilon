

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummarySystem {
  #[serde(rename = "system")]
  system: Option<Vec<::models::SummarySystemSystemItem>>
}

