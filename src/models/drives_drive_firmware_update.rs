

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrivesDriveFirmwareUpdate {
  /// A list of errors encountered by the individual nodes involved in this request, or an empty list if there were no errors.
  #[serde(rename = "errors")]
  errors: Option<Vec<::models::NodeDrivesPurposelistError>>,
  /// The responses from the individual nodes involved in this request.
  #[serde(rename = "nodes")]
  nodes: Option<Vec<::models::DrivesDriveFirmwareUpdateNode>>,
  /// The total number of nodes responding.
  #[serde(rename = "total")]
  total: Option<i32>
}

