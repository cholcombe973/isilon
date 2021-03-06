/// EventChannel : Named channel through which alerts can be delivered.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventChannel {
    /// Nodes (LNNs) that can be masters for this channel
    #[serde(rename = "allowed_nodes")]
    pub allowed_nodes: Option<Vec<i32>>,
    /// Channel is to be used or not
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Nodes (LNNs) that can NOT be the masters for this channel
    #[serde(rename = "excluded_nodes")]
    pub excluded_nodes: Option<Vec<i32>>,
    /// Parameters to be used for an smtp channel
    #[serde(rename = "parameters")]
    pub parameters: Option <crate::models::EventChannelParameters>,
    /// Channel is a pre-defined system channel
    #[serde(rename = "system")]
    pub system: Option<bool>,
    /// The mechanism used by the channel
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
