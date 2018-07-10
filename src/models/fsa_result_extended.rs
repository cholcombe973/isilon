#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsaResultExtended {
    /// True if the result is pinned to prevent automatic removal.
    #[serde(rename = "pinned")]
    pub pinned: bool,
    /// Unix Epoch time of start of results collection job.
    #[serde(rename = "begin_time")]
    pub begin_time: i32,
    /// Path to results database.
    #[serde(rename = "content_path")]
    pub content_path: Option<String>,
    /// Resource to call with DELETE to remove results..
    #[serde(rename = "delete_link")]
    pub delete_link: Option<String>,
    /// Unix Epoch time of end of results collection job.
    #[serde(rename = "end_time")]
    pub end_time: i32,
    /// State of the result set.
    #[serde(rename = "fsa_state")]
    pub fsa_state: String,
    /// The system generated result set ID.
    #[serde(rename = "id")]
    pub id: i32,
    /// State information about the FSA Job.
    #[serde(rename = "job_state")]
    pub job_state: Vec<String>,
    /// Resource to call to get result properties.
    #[serde(rename = "properties_link")]
    pub properties_link: String,
    /// Size of the result set database in bytes.
    #[serde(rename = "size")]
    pub size: i32,
    /// FSA version used to create result set.
    #[serde(rename = "version")]
    pub version: i32,
}
