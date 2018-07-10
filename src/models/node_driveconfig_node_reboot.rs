#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfigNodeReboot {
    /// Indicates whether or not to reboot the node due to a lost chassis.
    #[serde(rename = "chassis_loss")]
    pub chassis_loss: Option<bool>,
    /// Indicates whether or not to reboot the node if no drives are present.
    #[serde(rename = "none_present")]
    pub none_present: Option<bool>,
}
