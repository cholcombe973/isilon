#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStateNodeServicelight {
    /// The node service light state (True = on).
    #[serde(rename = "enabled")]
    pub enabled: bool,
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
