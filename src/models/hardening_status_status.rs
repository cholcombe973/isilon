#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardeningStatusStatus {
    /// Status text containing cluster-level and nodewise hardening status. Also includes hardening profile if hardening is enabled on at least one node.
    #[serde(rename = "message")]
    pub message: Option<String>,
}
