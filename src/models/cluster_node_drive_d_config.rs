#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeDriveDConfig {
    /// Configuration setting for drive alerts.
    #[serde(rename = "alert")]
    pub alert: Option <crate::models::NodeDriveconfigNodeAlert>,
    /// Configuration settings for drive formatting.
    #[serde(rename = "allow")]
    pub allow: Option <crate::models::NodeDriveconfigNodeAllow>,
    /// Configuration settings for automatic replacement recognition (ARR).
    #[serde(rename = "automatic_replacement_recognition")]
    pub automatic_replacement_recognition:
        Option <crate::models::NodeDriveconfigNodeAutomaticReplacementRecognition>,
    /// Configuration settings for drive statistics logs.
    #[serde(rename = "log")]
    pub log: Option <crate::models::NodeDriveconfigNodeLog>,
    /// Configuration settings for a node reboot due to a drive error.
    #[serde(rename = "reboot")]
    pub reboot: Option <crate::models::NodeDriveconfigNodeReboot>,
    /// Configuration settings for sleeping the drive daemon before node is rescanned.
    #[serde(rename = "spin_wait")]
    pub spin_wait: Option <crate::models::NodeDriveconfigNodeSpinWait>,
    /// Configuration settings to evaluate a drive stall.
    #[serde(rename = "stall")]
    pub stall: Option <crate::models::NodeDriveconfigNodeStall>,
}
