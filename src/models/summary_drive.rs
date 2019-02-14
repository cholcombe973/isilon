#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryDrive {
    #[serde(rename = "drive")]
    pub drive: Option<Vec <crate::models::SummaryDriveDriveItem>>,
}
