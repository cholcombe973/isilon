#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeState {
    /// Node readonly state.
    #[serde(rename = "readonly")]
    pub readonly: Option<::models::Empty>,
    /// Node service light state.
    #[serde(rename = "servicelight")]
    pub servicelight: Option<::models::ClusterNodeStateServicelight>,
    /// Node smartfail state.
    #[serde(rename = "smartfail")]
    pub smartfail: Option<::models::ClusterNodeStateSmartfail>,
}
