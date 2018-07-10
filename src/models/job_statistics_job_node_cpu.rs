

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeCpu {
  /// The average CPU utilization of the job on this node.
  #[serde(rename = "average")]
  average: Option<f32>,
  /// The current CPU utilization of the job on this node.
  #[serde(rename = "current")]
  current: f32,
  /// The maximum CPU utilization of the job on this node.
  #[serde(rename = "maximum")]
  maximum: f32,
  /// The minimum CPU utilization of the job on this node.
  #[serde(rename = "minimum")]
  minimum: f32
}

