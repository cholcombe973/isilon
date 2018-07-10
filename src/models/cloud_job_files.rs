#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudJobFiles {
    /// The file filtering logic to find files for this job
    #[serde(rename = "file_matching_pattern")]
    pub file_matching_pattern: Option<::models::Empty>,
    #[serde(rename = "names")]
    pub names: Option<Vec<::models::CloudJobFilesName>>,
    /// The total number of files addressed by this job
    #[serde(rename = "total")]
    pub total: Option<i32>,
    /// The number of canceled files
    #[serde(rename = "total_canceled")]
    pub total_canceled: Option<i32>,
    /// The number of files which failed
    #[serde(rename = "total_failed")]
    pub total_failed: Option<i32>,
    /// The number of files pending action
    #[serde(rename = "total_pending")]
    pub total_pending: Option<i32>,
    /// The number of files currently being processed
    #[serde(rename = "total_processing")]
    pub total_processing: Option<i32>,
    /// The total number of files successfully completed
    #[serde(rename = "total_succeeded")]
    pub total_succeeded: Option<i32>,
}
