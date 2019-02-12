#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNode {
    ///
    #[serde(rename = "cpu")]
    pub cpu:crate::models::JobStatisticsJobNodeCpu,
    ///
    #[serde(rename = "io")]
    pub io:crate::models::JobStatisticsJobNodeIo,
    ///
    #[serde(rename = "memory")]
    pub memory:crate::models::JobStatisticsJobNodeMemory,
    /// The devid of the node.
    #[serde(rename = "node")]
    pub node: i32,
    /// The process ID of the job on this node.
    #[serde(rename = "pid")]
    pub pid: i32,
    /// The number of workers for this job on this node.
    #[serde(rename = "total_workers")]
    pub total_workers: i32,
    #[serde(rename = "workers")]
    pub workers: Vec <crate::models::JobStatisticsJobNodeWorker>,
}
