

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpLogsNode {
  /// The page on this node's NDMP log file being returned.
  #[serde(rename = "current_page")]
  current_page: Option<String>,
  /// Error message, if the HTTP status returned from this node was not 200.
  #[serde(rename = "error")]
  error: Option<String>,
  /// Node ID of the node reporting this information.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Logical node number of the node reporting this information.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// The log file entries from the current page on this node.
  #[serde(rename = "logs")]
  logs: Option<String>,
  /// The highest page number in this node's NDMP log file.
  #[serde(rename = "max_page")]
  max_page: Option<i32>,
  /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
  #[serde(rename = "status")]
  status: Option<i32>
}

