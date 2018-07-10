#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHardwareFastNode {
    /// Node ID (Device Number) of this node.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Logical Node Number (LNN) of this node.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// Isilon product name.
    #[serde(rename = "product")]
    pub product: Option<String>,
    /// Serial number of this node.
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    /// Storage class of this node (storage or diskless).
    #[serde(rename = "storage_class")]
    pub storage_class: Option<String>,
}
