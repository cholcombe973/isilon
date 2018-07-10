

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrivesDriveFirmwareUpdateNode {
  /// Error message, if the HTTP status returned from this node was not 200.
  #[serde(rename = "error")]
  error: Option<String>,
  /// Node ID of the node reporting this information.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Logical node number of the node reporting this information.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// Drive firmware update status information.
  #[serde(rename = "status")]
  status: Option<::models::DrivesDriveFirmwareUpdateNodeStatus>
}

