

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeMemoryPhysical {
  /// The average physical memory utilization of the job on this node, in KB.
  #[serde(rename = "average")]
  average: Option<f32>,
  /// The current physical memory utilization of the job on this node, in KB.
  #[serde(rename = "current")]
  current: f32,
  /// The maximum physical memory utilization of the job on this node, in KB.
  #[serde(rename = "maximum")]
  maximum: f32,
  /// The minimum physical memory utilization of the job on this node, in KB.
  #[serde(rename = "minimum")]
  minimum: f32
}

