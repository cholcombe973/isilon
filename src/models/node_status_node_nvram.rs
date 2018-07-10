#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodeNvram {
    /// This node's NVRAM battery status information.
    #[serde(rename = "batteries")]
    pub batteries: Option<Vec<::models::NodeStatusNodeNvramBattery>>,
    /// This node's NVRAM battery count.
    #[serde(rename = "battery_count")]
    pub battery_count: Option<i32>,
    /// This node's NVRAM battery charge status, as a color.
    #[serde(rename = "charge_status")]
    pub charge_status: Option<String>,
    /// This node's NVRAM battery charge status, as a number.
    #[serde(rename = "charge_status_number")]
    pub charge_status_number: Option<i32>,
    /// This node's NVRAM device name with path.
    #[serde(rename = "device")]
    pub device: Option<String>,
    /// This node has NVRAM.
    #[serde(rename = "present")]
    pub present: Option<bool>,
    /// This node has NVRAM with flash storage.
    #[serde(rename = "present_flash")]
    pub present_flash: Option<bool>,
    /// The size of the NVRAM, in bytes.
    #[serde(rename = "present_size")]
    pub present_size: Option<i32>,
    /// This node's NVRAM type.
    #[serde(rename = "present_type")]
    pub present_type: Option<String>,
    /// This node's current ship mode state for NVRAM batteries.
    #[serde(rename = "ship_mode")]
    pub ship_mode: Option<i32>,
    /// This node supports NVRAM.
    #[serde(rename = "supported")]
    pub supported: Option<bool>,
    /// This node supports NVRAM with flash storage.
    #[serde(rename = "supported_flash")]
    pub supported_flash: Option<bool>,
    /// The maximum size of the NVRAM, in bytes.
    #[serde(rename = "supported_size")]
    pub supported_size: Option<i32>,
    /// This node's supported NVRAM type.
    #[serde(rename = "supported_type")]
    pub supported_type: Option<String>,
}
