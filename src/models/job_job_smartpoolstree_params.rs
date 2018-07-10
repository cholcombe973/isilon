

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobSmartpoolstreeParams {
  /// Skip processing of regular files.
  #[serde(rename = "directory_only")]
  directory_only: Option<bool>,
  /// Calculate what would be done (dry run).
  #[serde(rename = "nop")]
  nop: Option<bool>,
  /// Apply policies but skip restriping.
  #[serde(rename = "policy_only")]
  policy_only: Option<bool>,
  /// Process children, recursively.
  #[serde(rename = "recurse")]
  recurse: Option<bool>
}

