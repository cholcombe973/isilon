

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfigNode {
  /// Configuration setting for drive alerts.
  #[serde(rename = "alert")]
  alert: Option<::models::NodeDriveconfigNodeAlert>,
  /// Configuration settings for drive formatting.
  #[serde(rename = "allow")]
  allow: Option<::models::NodeDriveconfigNodeAllow>,
  /// Configuration settings for automatic replacement recognition (ARR).
  #[serde(rename = "automatic_replacement_recognition")]
  automatic_replacement_recognition: Option<::models::NodeDriveconfigNodeAutomaticReplacementRecognition>,
  /// Node ID (Device Number) of this node.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Logical Node Number (LNN) of this node.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// Configuration settings for drive statistics logs.
  #[serde(rename = "log")]
  log: Option<::models::NodeDriveconfigNodeLog>,
  /// Configuration settings for a node reboot due to a drive error.
  #[serde(rename = "reboot")]
  reboot: Option<::models::NodeDriveconfigNodeReboot>,
  /// Configuration settings for sleeping the drive daemon before node is rescanned.
  #[serde(rename = "spin_wait")]
  spin_wait: Option<::models::NodeDriveconfigNodeSpinWait>,
  /// Configuration settings to evaluate a drive stall.
  #[serde(rename = "stall")]
  stall: Option<::models::NodeDriveconfigNodeStall>
}

