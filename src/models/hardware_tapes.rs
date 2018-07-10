/// HardwareTapes : Get list Tape and Changer devices

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareTapes {
    /// Information of Tape/MC device
    #[serde(rename = "devices")]
    pub devices: Option<::models::HardwareTapesDevices>,
    /// Resume string returned by previous query.
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    /// The number of devices
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
