

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZonesSummarySummary {
  /// The zone base path
  #[serde(rename = "path")]
  path: String
}

