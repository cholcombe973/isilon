

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStateServicelightNode {
  /// The node service light state (True = on).
  #[serde(rename = "enabled")]
  enabled: bool,
  /// Error message, if the HTTP status returned from this node was not 200.
  #[serde(rename = "error")]
  error: Option<String>,
  /// Node ID of the node reporting this information.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Logical node number of the node reporting this information.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// This node has a service light.
  #[serde(rename = "present")]
  present: Option<bool>,
  /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
  #[serde(rename = "status")]
  status: Option<i32>,
  /// This node supports a service light.
  #[serde(rename = "supported")]
  supported: Option<bool>,
  /// The node service light state is valid (False = Error).
  #[serde(rename = "valid")]
  valid: Option<bool>
}

