/// CloudJobCreateParams : A cloud job for archiving or recalling files or restoring COI

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudJobCreateParams {
    /// The names of accounts for COI restore
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<String>>,
    /// Directories addressed by this job
    #[serde(rename = "directories")]
    pub directories: Option<Vec<String>>,
    /// The new expiration date in seconds
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<i32>,
    /// The file filtering logic to find files for this job. (Only applicable for 'recall' jobs)
    #[serde(rename = "file_matching_pattern")]
    pub file_matching_pattern: Option <crate::models::Empty>,
    /// Filenames addressed by this job
    #[serde(rename = "files")]
    pub files: Option<Vec<String>>,
    /// The name of an existing cloudpool policy to apply to this job. (Only applicable for 'archive' jobs)
    #[serde(rename = "policy")]
    pub policy: Option<String>,
    /// The type of cloud action to be performed by this job.
    #[serde(rename = "type")]
    pub _type: String,
}
