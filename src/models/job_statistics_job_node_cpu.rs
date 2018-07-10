#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeCpu {
    /// The average CPU utilization of the job on this node.
    #[serde(rename = "average")]
    pub average: Option<f32>,
    /// The current CPU utilization of the job on this node.
    #[serde(rename = "current")]
    pub current: f32,
    /// The maximum CPU utilization of the job on this node.
    #[serde(rename = "maximum")]
    pub maximum: f32,
    /// The minimum CPU utilization of the job on this node.
    #[serde(rename = "minimum")]
    pub minimum: f32,
}
