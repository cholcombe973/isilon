#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareStatus {
    #[serde(rename = "status")]
    pub status: Option<Vec <crate::models::HardwareStatusStatusItem>>,
}
