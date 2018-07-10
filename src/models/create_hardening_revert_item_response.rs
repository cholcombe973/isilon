#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHardeningRevertItemResponse {
    /// Message text indicating if hardening revert operation started successfully or failed or start.
    #[serde(rename = "message")]
    pub message: Option<String>,
}
