

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsGatherStatusGatherStatus {
  /// The current status of the process
  #[serde(rename = "Active_Status")]
  active_status: String,
  /// The previous status of the process
  #[serde(rename = "Last_Status")]
  last_status: String
}

