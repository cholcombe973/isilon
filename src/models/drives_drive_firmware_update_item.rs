/// DrivesDriveFirmwareUpdateItem : Drive firmware update information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrivesDriveFirmwareUpdateItem {
    /// Indicates whether this is a cluster wide drive firwmare update or not
    #[serde(rename = "cluster_wide")]
    pub cluster_wide: bool,
}
