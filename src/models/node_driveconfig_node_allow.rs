#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfigNodeAllow {
    /// Allow formatting a drive model with unknown firmware.
    #[serde(rename = "format_unknown_firmware")]
    pub format_unknown_firmware: Option<bool>,
    /// Allow formatting an unknown drive model.
    #[serde(rename = "format_unknown_model")]
    pub format_unknown_model: Option<bool>,
}
