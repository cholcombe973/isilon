

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterIdentityLogon {
  /// The message of the day.
  #[serde(rename = "motd")]
  motd: String,
  /// The header to the message of the day.
  #[serde(rename = "motd_header")]
  motd_header: String
}

