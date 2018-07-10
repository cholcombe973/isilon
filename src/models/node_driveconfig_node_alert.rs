#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfigNodeAlert {
    /// Send alerts for unknown drive firmware.
    #[serde(rename = "unknown_firmware")]
    pub unknown_firmware: Option<bool>,
    /// Send alerts for unknown drive model.
    #[serde(rename = "unknown_model")]
    pub unknown_model: Option<bool>,
}
