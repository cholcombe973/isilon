

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsaResultsExtended {
  #[serde(rename = "results")]
  results: Option<Vec<::models::FsaResultExtended>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

