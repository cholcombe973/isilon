

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStatsDisk {
  /// Disk iops
  #[serde(rename = "iops")]
  iops: f32,
  /// Disk reads
  #[serde(rename = "read")]
  read: f32,
  /// Disk writes
  #[serde(rename = "write")]
  write: f32
}

