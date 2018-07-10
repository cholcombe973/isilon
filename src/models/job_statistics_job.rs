#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJob {
    /// The job ID.
    #[serde(rename = "job_id")]
    pub job_id: i32,
    #[serde(rename = "nodes")]
    pub nodes: Vec<::models::JobStatisticsJobNode>,
    /// The current phase of the job.
    #[serde(rename = "phase")]
    pub phase: i32,
    /// The number of nodes participating in the job.
    #[serde(rename = "total_nodes")]
    pub total_nodes: i32,
}
