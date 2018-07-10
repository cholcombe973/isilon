

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelistLinsCtime {
  /// Nanoseconds component of timespec.
  #[serde(rename = "nsec")]
  nsec: Option<i32>,
  /// Seconds component of timespec.
  #[serde(rename = "sec")]
  sec: i32
}

