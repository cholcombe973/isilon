

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSharesExtended {
  #[serde(rename = "shares")]
  shares: Option<Vec<::models::SmbShareExtended>>,
  /// An identifier for a set of shares.
  #[serde(rename = "digest")]
  digest: Option<String>,
  /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

