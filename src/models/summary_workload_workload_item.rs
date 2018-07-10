

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryWorkloadWorkloadItem {
  /// The number (across all cores) of micro-seconds per second.
  #[serde(rename = "cpu")]
  cpu: f32,
  /// The canonical name for the job followed by phase in brackets, ie. 'AVscan[1]', etc...
  #[serde(rename = "job_type")]
  job_type: Option<String>,
  /// L2 cache hits per second.
  #[serde(rename = "l2")]
  l2: f32,
  /// L3 cache hits per second.
  #[serde(rename = "l3")]
  l3: f32,
  /// The node on which the operation was performed.
  #[serde(rename = "node")]
  node: f32,
  /// Disk read operations per second.
  #[serde(rename = "reads")]
  reads: f32,
  /// The process name, job ID, etc...
  #[serde(rename = "system_name")]
  system_name: Option<String>,
  /// Disk write operations per second.
  #[serde(rename = "writes")]
  writes: f32
}

