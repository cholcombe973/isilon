

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NtpSettingsSettings {
  /// Number of nodes that will contact the NTP servers.
  #[serde(rename = "chimers")]
  chimers: Option<i32>,
  /// Node number (LNN) for nodes excluded from chimer duty.
  #[serde(rename = "excluded")]
  excluded: Option<Vec<String>>,
  /// Path to NTP key file within /ifs.
  #[serde(rename = "key_file")]
  key_file: Option<String>
}

