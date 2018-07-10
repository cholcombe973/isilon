#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardeningState {
    /// The state of hardening operation on the cluster.
    #[serde(rename = "state")]
    pub state: Option<::models::HardeningStateState>,
}
