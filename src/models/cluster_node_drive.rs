

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeDrive {
  /// Numerical representation of this drive's bay.
  #[serde(rename = "baynum")]
  baynum: Option<i32>,
  /// Number of blocks on this drive.
  #[serde(rename = "blocks")]
  blocks: Option<i32>,
  /// The chassis number which contains this drive.
  #[serde(rename = "chassis")]
  chassis: Option<i32>,
  /// This drive's device name.
  #[serde(rename = "devname")]
  devname: Option<String>,
  /// Drive firmware information.
  #[serde(rename = "firmware")]
  firmware: Option<::models::NodeDrivesNodeDriveFirmware>,
  /// Drive_d's handle representation for this drive
  #[serde(rename = "handle")]
  handle: Option<i32>,
  /// String representtation of this drive's interface type.
  #[serde(rename = "interface_type")]
  interface_type: Option<String>,
  /// This drive's logical drive number in IFS.
  #[serde(rename = "lnum")]
  lnum: Option<i32>,
  /// String representation of this drive's physical location.
  #[serde(rename = "locnstr")]
  locnstr: Option<String>,
  /// Size of a logical block on this drive.
  #[serde(rename = "logical_block_length")]
  logical_block_length: Option<i32>,
  /// String representation of this drive's media type.
  #[serde(rename = "media_type")]
  media_type: Option<String>,
  /// This drive's manufacturer and model.
  #[serde(rename = "model")]
  model: Option<String>,
  /// Size of a physical block on this drive.
  #[serde(rename = "physical_block_length")]
  physical_block_length: Option<i32>,
  /// Indicates whether this drive is physically present in the node.
  #[serde(rename = "present")]
  present: Option<bool>,
  /// This drive's purpose in the DRV state machine.
  #[serde(rename = "purpose")]
  purpose: Option<String>,
  /// Description of this drive's purpose.
  #[serde(rename = "purpose_description")]
  purpose_description: Option<String>,
  /// Serial number for this drive.
  #[serde(rename = "serial")]
  serial: Option<String>,
  /// This drive's state as presented to the UI.
  #[serde(rename = "ui_state")]
  ui_state: Option<String>,
  /// The drive's 'worldwide name' from its NAA identifiers.
  #[serde(rename = "wwn")]
  wwn: Option<String>,
  /// This drive's x-axis grid location.
  #[serde(rename = "x_loc")]
  x_loc: Option<i32>,
  /// This drive's y-axis grid location.
  #[serde(rename = "y_loc")]
  y_loc: Option<i32>
}

