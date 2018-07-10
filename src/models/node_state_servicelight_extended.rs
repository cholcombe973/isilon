
/// NodeStateServicelightExtended : Node service light state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStateServicelightExtended {
  /// The node service light state (True = on).
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// This node has a service light.
  #[serde(rename = "present")]
  present: Option<bool>,
  /// This node supports a service light.
  #[serde(rename = "supported")]
  supported: Option<bool>,
  /// The node service light state is valid (False = Error).
  #[serde(rename = "valid")]
  valid: Option<bool>
}

