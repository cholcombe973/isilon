#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfigNodeSpinWait {
    /// Seconds to wait between enabling a bay and checking for an inserted drive.
    #[serde(rename = "check_drive")]
    pub check_drive: Option<i32>,
    /// Seconds to wait between enabling a bay and enabling another bay.
    #[serde(rename = "stagger")]
    pub stagger: Option<i32>,
}
