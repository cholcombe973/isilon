

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNode {
  /// 
  #[serde(rename = "cpu")]
  cpu: ::models::JobStatisticsJobNodeCpu,
  /// 
  #[serde(rename = "io")]
  io: ::models::JobStatisticsJobNodeIo,
  /// 
  #[serde(rename = "memory")]
  memory: ::models::JobStatisticsJobNodeMemory,
  /// The devid of the node.
  #[serde(rename = "node")]
  node: i32,
  /// The process ID of the job on this node.
  #[serde(rename = "pid")]
  pid: i32,
  /// The number of workers for this job on this node.
  #[serde(rename = "total_workers")]
  total_workers: i32,
  #[serde(rename = "workers")]
  workers: Vec<::models::JobStatisticsJobNodeWorker>
}

