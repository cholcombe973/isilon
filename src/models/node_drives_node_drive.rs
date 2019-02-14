#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDrivesNodeDrive {
    /// The name of the bay group this drive belongs to.
    #[serde(rename = "bay_group")]
    pub bay_group: Option<String>,
    /// Numerical representation of this drive's bay.
    #[serde(rename = "baynum")]
    pub baynum: Option<i32>,
    /// Number of blocks on this drive.
    #[serde(rename = "blocks")]
    pub blocks: Option<i32>,
    /// The chassis number which contains this drive.
    #[serde(rename = "chassis")]
    pub chassis: Option<i32>,
    /// This drive's device name.
    #[serde(rename = "devname")]
    pub devname: Option<String>,
    /// Drive firmware information.
    #[serde(rename = "firmware")]
    pub firmware: Option <crate::models::NodeDrivesNodeDriveFirmware>,
    /// Drive_d's handle representation for this drive
    #[serde(rename = "handle")]
    pub handle: Option<i32>,
    /// String representtation of this drive's interface type.
    #[serde(rename = "interface_type")]
    pub interface_type: Option<String>,
    /// This drive's logical drive number in IFS.
    #[serde(rename = "lnum")]
    pub lnum: Option<i32>,
    /// String representation of this drive's physical location.
    #[serde(rename = "locnstr")]
    pub locnstr: Option<String>,
    /// Size of a logical block on this drive.
    #[serde(rename = "logical_block_length")]
    pub logical_block_length: Option<i32>,
    /// String representation of this drive's media type.
    #[serde(rename = "media_type")]
    pub media_type: Option<String>,
    /// This drive's manufacturer and model.
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// This drive's current outstanding actions. For example, \"add\" or \"firmware_update\".
    #[serde(rename = "pending_actions")]
    pub pending_actions: Option<Vec<String>>,
    /// Size of a physical block on this drive.
    #[serde(rename = "physical_block_length")]
    pub physical_block_length: Option<i32>,
    /// Indicates whether this drive is physically present in the node.
    #[serde(rename = "present")]
    pub present: Option<bool>,
    /// This drive's purpose in the DRV state machine.
    #[serde(rename = "purpose")]
    pub purpose: Option<String>,
    /// Description of this drive's purpose.
    #[serde(rename = "purpose_description")]
    pub purpose_description: Option<String>,
    /// Serial number for this drive.
    #[serde(rename = "serial")]
    pub serial: Option<String>,
    /// This drive's state as presented to the UI.
    #[serde(rename = "ui_state")]
    pub ui_state: Option<String>,
    /// The drive's 'worldwide name' from its NAA identifiers.
    #[serde(rename = "wwn")]
    pub wwn: Option<String>,
    /// This drive's x-axis grid location.
    #[serde(rename = "x_loc")]
    pub x_loc: Option<i32>,
    /// This drive's y-axis grid location.
    #[serde(rename = "y_loc")]
    pub y_loc: Option<i32>,
}
