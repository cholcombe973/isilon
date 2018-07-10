
/// ClusterFirmwareUpgradeItem : The settings necessary to start a firmware upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterFirmwareUpgradeItem {
  /// Exclude the specified devices in the firmware upgrade.
  #[serde(rename = "exclude_device")]
  exclude_device: Option<String>,
  /// Exclude the specified device type in the firmware upgrade.
  #[serde(rename = "exclude_type")]
  exclude_type: Option<String>,
  /// Include the specified devices in the firmware upgrade.
  #[serde(rename = "include_device")]
  include_device: Option<String>,
  /// Include the specified device type in the firmware upgrade.
  #[serde(rename = "include_type")]
  include_type: Option<String>,
  /// Do not burn the firmware.
  #[serde(rename = "no_burn")]
  no_burn: Option<bool>,
  /// Do not reboot the node after an upgrade
  #[serde(rename = "no_reboot")]
  no_reboot: Option<bool>,
  /// Do not verify the firmware upgrade after an upgrade.
  #[serde(rename = "no_verify")]
  no_verify: Option<bool>,
  /// The nodes scheduled for upgrade. Order in array determines queue position number. 'All' and null option will upgrade all nodes in <lnn> order.
  #[serde(rename = "nodes_to_upgrade")]
  nodes_to_upgrade: Option<Vec<i32>>
}

