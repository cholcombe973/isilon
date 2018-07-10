

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSledsNodeSled {
  /// Boolean to indicate whether or not the sled is safe to remove.
  #[serde(rename = "is_removeable")]
  is_removeable: bool,
  /// The sled letter which OneFS uses to refer to this sled in the node.
  #[serde(rename = "sled_letter")]
  sled_letter: String,
  /// The state of physical presence of a sled.
  #[serde(rename = "sled_state")]
  sled_state: String
}

