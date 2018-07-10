#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfigNodeLog {
    /// Indicates whether or not to log the drive statistics.
    #[serde(rename = "drive_stats")]
    pub drive_stats: Option<bool>,
}
