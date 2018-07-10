

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventChannelCreateParams {
  /// Nodes (LNNs) that can be masters for this channel
  #[serde(rename = "allowed_nodes")]
  allowed_nodes: Option<Vec<i32>>,
  /// Channel is to be used or not
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// Nodes (LNNs) that can NOT be the masters for this channel
  #[serde(rename = "excluded_nodes")]
  excluded_nodes: Option<Vec<i32>>,
  /// Parameters to be used for an smtp channel
  #[serde(rename = "parameters")]
  parameters: Option<::models::EventChannelParameters>,
  /// Channel is a pre-defined system channel
  #[serde(rename = "system")]
  system: Option<bool>,
  /// The mechanism used by the channel
  #[serde(rename = "type")]
  _type: String,
  /// Unique identifier.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Channel name,  may not contain /, max length 254.
  #[serde(rename = "name")]
  name: String
}

