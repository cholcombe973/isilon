#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeState {
    /// Node readonly state.
    #[serde(rename = "readonly")]
    pub readonly: Option <crate::models::Empty>,
    /// Node service light state.
    #[serde(rename = "servicelight")]
    pub servicelight: Option <crate::models::ClusterNodeStateServicelight>,
    /// Node smartfail state.
    #[serde(rename = "smartfail")]
    pub smartfail: Option <crate::models::ClusterNodeStateSmartfail>,
}
