

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSession {
  /// Unique display id.
  #[serde(rename = "id")]
  id: Option<String>,
  /// Text information from probing the session
  #[serde(rename = "probe_info")]
  probe_info: Option<String>,
  /// session ID
  #[serde(rename = "session")]
  session: Option<String>
}

