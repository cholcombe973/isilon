#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportsScansReport {
    /// The number of bytes sent to the virus definition server to be scanned.
    #[serde(rename = "bytes_sent")]
    pub bytes_sent: Option<i32>,
    /// The length of time the job ran for.
    #[serde(rename = "duration")]
    pub duration: Option<i32>,
    #[serde(rename = "end")]
    pub end: Option<i32>,
    /// The number of files scanned.
    #[serde(rename = "files")]
    pub files: Option<i32>,
    /// A unique identifier for the report.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The number of infections found.
    #[serde(rename = "infections")]
    pub infections: Option<i32>,
    #[serde(rename = "job_id")]
    pub job_id: Option<i32>,
    /// The id of the policy that this scan job executed.
    #[serde(rename = "policy_id")]
    pub policy_id: Option<String>,
    /// The cumulative size of the files scanned.
    #[serde(rename = "size")]
    pub size: Option<i32>,
    #[serde(rename = "start")]
    pub start: Option<i32>,
    /// The state of the job.
    #[serde(rename = "status")]
    pub status: Option<String>,
}
