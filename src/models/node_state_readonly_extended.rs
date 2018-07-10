
/// NodeStateReadonlyExtended : Node readonly state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStateReadonlyExtended {
  /// The current read-only mode allowed status for the node.
  #[serde(rename = "allowed")]
  allowed: Option<bool>,
  /// The current read-only user mode status for the node. NOTE: If read-only mode is currently disallowed for this node, it will remain read/write until read-only mode is allowed again. This value only sets or clears any user-specified requests for read-only mode. If the node has been placed into read-only mode by the system, it will remain in read-only mode until the system conditions which triggered read-only mode have cleared.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// The current read-only mode status for the node.
  #[serde(rename = "mode")]
  mode: Option<bool>,
  /// The current read-only mode status description for the node.
  #[serde(rename = "status")]
  status: Option<String>,
  /// The read-only state values are valid (False = Error).
  #[serde(rename = "valid")]
  valid: Option<bool>,
  /// The current read-only value (enumerated bitfield) for the node.
  #[serde(rename = "value")]
  value: Option<i32>
}

