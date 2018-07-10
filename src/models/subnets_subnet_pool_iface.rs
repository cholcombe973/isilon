#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetsSubnetPoolIface {
    /// A string that defines an interface name.
    #[serde(rename = "iface")]
    pub iface: Option<String>,
    /// Logical Node Number.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
}
