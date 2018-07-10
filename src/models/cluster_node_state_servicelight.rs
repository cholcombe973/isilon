

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeStateServicelight {
  /// The node service light state (True = on).
  #[serde(rename = "enabled")]
  enabled: bool
}

