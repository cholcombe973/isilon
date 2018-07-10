

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareStatus {
  #[serde(rename = "status")]
  status: Option<Vec<::models::HardwareStatusStatusItem>>
}

