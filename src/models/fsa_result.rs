

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsaResult {
  /// True if the result is pinned to prevent automatic removal.
  #[serde(rename = "pinned")]
  pinned: bool
}

