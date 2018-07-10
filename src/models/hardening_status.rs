#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardeningStatus {
    /// Hardening Status of the cluster.
    #[serde(rename = "status")]
    pub status: Option<::models::HardeningStatusStatus>,
}
