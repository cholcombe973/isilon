/// NodeStateServicelightExtended : Node service light state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStateServicelightExtended {
    /// The node service light state (True = on).
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// This node has a service light.
    #[serde(rename = "present")]
    pub present: Option<bool>,
    /// This node supports a service light.
    #[serde(rename = "supported")]
    pub supported: Option<bool>,
    /// The node service light state is valid (False = Error).
    #[serde(rename = "valid")]
    pub valid: Option<bool>,
}
