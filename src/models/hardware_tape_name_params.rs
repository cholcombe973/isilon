#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareTapeNameParams {
    /// The name of the device
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Set the device state to close
    #[serde(rename = "state")]
    pub state: Option<String>,
}
