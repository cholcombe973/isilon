

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WormCreateParams {
  /// Specifies whether to commit the file to a WORM state after the retention date is set.
  #[serde(rename = "commit_to_worm")]
  commit_to_worm: Option<bool>,
  /// Specifies the retention expiration date string in Coordinated Universal Time (UTC/GMT).
  #[serde(rename = "worm_retention_date")]
  worm_retention_date: Option<String>
}

