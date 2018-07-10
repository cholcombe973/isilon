

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareFcportsNode {
  /// Error message, if the HTTP status returned from this node was not 200.
  #[serde(rename = "error")]
  error: Option<String>,
  /// A list of the FC ports on this node.
  #[serde(rename = "fcports")]
  fcports: Option<Vec<::models::HardwareFcportsNodeFcport>>,
  /// Node ID of the node reporting this information.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Logical node number of the node reporting this information.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
  #[serde(rename = "status")]
  status: Option<i32>
}

