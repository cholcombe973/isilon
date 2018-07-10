

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeState {
  /// Node readonly state.
  #[serde(rename = "readonly")]
  readonly: Option<::models::Empty>,
  /// Node service light state.
  #[serde(rename = "servicelight")]
  servicelight: Option<::models::ClusterNodeStateServicelight>,
  /// Node smartfail state.
  #[serde(rename = "smartfail")]
  smartfail: Option<::models::ClusterNodeStateSmartfail>
}

