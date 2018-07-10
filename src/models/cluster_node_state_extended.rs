

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeStateExtended {
  /// Node readonly state.
  #[serde(rename = "readonly")]
  readonly: Option<::models::NodeStateReadonlyExtended>,
  /// Node service light state.
  #[serde(rename = "servicelight")]
  servicelight: Option<::models::NodeStateNodeServicelight>,
  /// Node smartfail state.
  #[serde(rename = "smartfail")]
  smartfail: Option<::models::NodeStateSmartfailExtended>
}

